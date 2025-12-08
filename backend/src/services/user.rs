use crate::{
  establish_connection,
  schema::{self, users},
  services::{create_default_data, log},
  util::{self, error::APIError},
};
use diesel::{
  deserialize::Queryable, prelude::Insertable, AggregateExpressionMethods, ExpressionMethods,
  JoinOnDsl, QueryDsl, RunQueryDsl,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use super::delete_all_user_sessions;

use dotenvy::dotenv;
use std::env;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateUser {
  #[validate(length(min = 1, max = 255))]
  pub name: String,
  #[validate(email)]
  pub email: String,
  #[validate(length(min = 7, max = 72))]
  pub password: String,
  pub invite: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct AuthUser {
  #[validate(email)]
  pub email: String,
  #[validate(length(min = 7, max = 72))]
  pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateUser {
  #[validate(length(min = 1, max = 255))]
  pub name: String,
  #[validate(email)]
  pub email: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdatePassword {
  #[validate(length(min = 7, max = 72))]
  pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Insertable, Queryable)]
pub struct User {
  pub id: String,
  pub created_at: i64,
  pub name: String,
  pub email: String,
  pub password: String,
  pub invite: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Queryable)]
pub struct UserDetails {
  pub id: String,
  pub created_at: i64,
  pub name: String,
  pub email: String,
  pub invite: Option<String>,
}

pub fn get_user_id(email: &str) -> Result<String, APIError> {
  let mut conn = match establish_connection() {
    Ok(connection) => connection,
    Err(_) => return Err(APIError::DatabaseError),
  };

  match schema::users::table
    .filter(schema::users::email.eq(email))
    .select(schema::users::id)
    .first(&mut conn)
  {
    Ok(id) => Ok(id),
    Err(_) => Err(APIError::UserNotFound),
  }
}

pub fn get_user(id: &str) -> Result<UserDetails, APIError> {
  let mut conn = match establish_connection() {
    Ok(connection) => connection,
    Err(_) => return Err(APIError::DatabaseError),
  };

  match schema::users::table
    .filter(schema::users::id.eq(&id))
    .select((
      schema::users::id,
      schema::users::created_at,
      schema::users::name,
      schema::users::email,
      schema::users::invite,
    ))
    .first(&mut conn)
  {
    Ok(user) => Ok(user),
    Err(_) => Err(APIError::UserNotFound),
  }
}

pub fn get_password_hash(id: &str) -> Result<String, APIError> {
  let mut conn = match establish_connection() {
    Ok(connection) => connection,
    Err(_) => return Err(APIError::DatabaseError),
  };

  match schema::users::table
    .filter(schema::users::id.eq(&id))
    .first::<User>(&mut conn)
  {
    Ok(user) => Ok(user.password),
    Err(_) => Err(APIError::UserNotFound),
  }
}

pub fn create_user(user: CreateUser) -> Result<UserDetails, APIError> {
  match user.validate() {
    Ok(_) => (),
    Err(_) => return Err(APIError::BadRequest),
  }

  if get_user_id(&user.email).is_ok() {
    return Err(APIError::EmailAlreadyInUse);
  }

  let mut conn = match establish_connection() {
    Ok(connection) => connection,
    Err(_) => return Err(APIError::DatabaseError),
  };

  dotenv().ok();

  let cost = match env::var("BCRYPT_COST") {
    Ok(val) => match val.parse::<u32>() {
      Ok(parsed) => parsed,
      Err(_) => bcrypt::DEFAULT_COST,
    },
    Err(_) => bcrypt::DEFAULT_COST,
  };

  let password_hash = match bcrypt::hash(&user.password, cost) {
    Ok(hash) => hash,
    Err(_) => return Err(APIError::InternalServerError),
  };

  let user_details = UserDetails {
    id: Uuid::new_v4().to_string(),
    created_at: util::unix_time::unix_ms(),
    name: user.name,
    email: user.email,
    invite: user.invite,
  };

  let new_user = User {
    id: user_details.id.clone(),
    created_at: user_details.created_at,
    name: user_details.name.clone(),
    email: user_details.email.clone(),
    password: password_hash,
    invite: user_details.invite.clone(),
  };

  match diesel::insert_into(schema::users::table)
    .values(&new_user)
    .execute(&mut conn)
  {
    Ok(_) => (),
    Err(_) => return Err(APIError::DatabaseError),
  };

  match create_default_data(new_user.id.clone()) {
    Ok(_) => (),
    Err(_) => return Err(APIError::DatabaseError),
  };

  Ok(user_details)
}

pub fn delete_user(id: &str) -> Result<bool, APIError> {
  let mut conn = match establish_connection() {
    Ok(connection) => connection,
    Err(_) => return Err(APIError::DatabaseError),
  };

  match delete_all_user_sessions(id) {
    Ok(_) => (),
    Err(_) => return Err(APIError::DatabaseError),
  };

  match log::delete_all_user_data(id) {
    Ok(_) => (),
    Err(_) => return Err(APIError::DatabaseError),
  };

  match diesel::delete(schema::users::table.filter(schema::users::id.eq(id))).execute(&mut conn) {
    Ok(rows_affected) => Ok(rows_affected > 0),
    Err(_) => Err(APIError::DatabaseError),
  }
}

pub fn update_user(id: &str, user: UpdateUser) -> Result<bool, APIError> {
  match user.validate() {
    Ok(_) => (),
    Err(_) => return Err(APIError::BadRequest),
  }

  if let Ok(existing_user_id) = get_user_id(&user.email) {
    if existing_user_id != id {
      return Err(APIError::EmailAlreadyInUse);
    }
  }

  let mut conn = match establish_connection() {
    Ok(connection) => connection,
    Err(_) => return Err(APIError::DatabaseError),
  };

  match diesel::update(schema::users::table.filter(schema::users::id.eq(id)))
    .set((
      schema::users::name.eq(&user.name),
      schema::users::email.eq(&user.email),
    ))
    .execute(&mut conn)
  {
    Ok(rows_affected) => Ok(rows_affected > 0),
    Err(_) => Err(APIError::UserNotFound),
  }
}

pub fn update_password(id: &str, password: UpdatePassword) -> Result<bool, APIError> {
  let mut conn = match establish_connection() {
    Ok(connection) => connection,
    Err(_) => return Err(APIError::DatabaseError),
  };

  let password_hash = match bcrypt::hash(&password.password, bcrypt::DEFAULT_COST) {
    Ok(hash) => hash,
    Err(_) => return Err(APIError::InternalServerError),
  };

  match diesel::update(schema::users::table.filter(schema::users::id.eq(id)))
    .set(schema::users::password.eq(&password_hash))
    .execute(&mut conn)
  {
    Ok(rows_affected) => Ok(rows_affected > 0),
    Err(_) => Err(APIError::DatabaseError),
  }
}

pub fn user_count() -> Result<i64, APIError> {
  let mut conn = match establish_connection() {
    Ok(connection) => connection,
    Err(_) => return Err(APIError::DatabaseError),
  };

  match schema::users::table.count().get_result::<i64>(&mut conn) {
    Ok(count) => Ok(count),
    Err(_) => Err(APIError::DatabaseError),
  }
}

pub fn active_user_count(since_timestamp: i64) -> Result<i64, APIError> {
  let mut conn = match establish_connection() {
    Ok(connection) => connection,
    Err(_) => return Err(APIError::DatabaseError),
  };

  match schema::users::table
    .inner_join(schema::sessions::table.on(schema::users::id.eq(schema::sessions::user_id)))
    .filter(schema::sessions::accessed_at.ge(since_timestamp))
    .select(diesel::dsl::count(schema::users::id).aggregate_distinct())
    .first::<i64>(&mut conn)
  {
    Ok(count) => Ok(count),
    Err(_) => Err(APIError::DatabaseError),
  }
}

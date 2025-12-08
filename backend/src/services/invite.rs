use crate::{
  establish_connection,
  schema::{self, invites},
  util::error::APIError,
  util::generate_invite_code,
};
use diesel::{
  prelude::{Insertable, Queryable},
  ExpressionMethods, QueryDsl, RunQueryDsl,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Insertable, Queryable)]
pub struct Invite {
  pub id: String,
  pub created_at: i64,
  pub code: String,
  pub used: bool,
}

pub fn get_invite(code: &str) -> Result<Invite, APIError> {
  let mut conn = match establish_connection() {
    Ok(connection) => connection,
    Err(_) => return Err(APIError::DatabaseError),
  };

  match schema::invites::table
    .filter(schema::invites::code.eq(&code))
    .first(&mut conn)
  {
    Ok(invite) => Ok(invite),
    Err(_) => Err(APIError::InviteNotFound),
  }
}

pub fn use_invite(code: &str) -> Result<Invite, APIError> {
  let mut conn = match establish_connection() {
    Ok(connection) => connection,
    Err(_) => return Err(APIError::DatabaseError),
  };

  let invite = get_invite(code)?;

  if invite.used {
    return Err(APIError::InviteUsed);
  }

  match diesel::update(schema::invites::table.filter(schema::invites::code.eq(&code)))
    .set(schema::invites::used.eq(true))
    .get_result(&mut conn)
  {
    Ok(invite) => Ok(invite),
    Err(_) => Err(APIError::DatabaseError),
  }
}

pub fn generate_invite(code: Option<&str>) -> Result<Invite, APIError> {
  let mut conn = match establish_connection() {
    Ok(connection) => connection,
    Err(_) => return Err(APIError::DatabaseError),
  };

  let code = match code {
    Some(c) => match get_invite(c) {
      Ok(_) => generate_invite_code(),
      Err(_) => c.to_string(),
    },
    None => generate_invite_code(),
  };

  let new_invite = Invite {
    id: Uuid::new_v4().to_string(),
    created_at: crate::util::unix_time::unix_ms(),
    code,
    used: false,
  };

  match diesel::insert_into(schema::invites::table)
    .values(&new_invite)
    .execute(&mut conn)
  {
    Ok(_) => Ok(new_invite),
    Err(_) => Err(APIError::DatabaseError),
  }
}

use crate::{
  establish_connection,
  schema::tags,
  services::{category::get_category, get_user},
  util::{self, APIError, Color},
};
use diesel::{
  prelude::{Insertable, Queryable},
  ExpressionMethods, QueryDsl, RunQueryDsl,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Insertable, Queryable)]
#[diesel(table_name = tags)]
pub struct Tag {
  pub id: String,
  pub user_id: String,
  pub created_at: i64,
  pub name: String,
  pub color: String,
  pub category_id: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateTag {
  #[validate(length(min = 1, max = 255))]
  pub name: String,
  #[validate(length(min = 1, max = 16))]
  pub color: String,
  #[validate(length(min = 1, max = 255))]
  pub category_id: String,
  #[validate(length(min = 1, max = 255))]
  pub user_id: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct EditTag {
  #[validate(length(min = 1, max = 255))]
  pub id: String,
  #[validate(length(min = 1, max = 255))]
  pub name: String,
  #[validate(length(min = 1, max = 16))]
  pub color: String,
  #[validate(length(min = 1, max = 255))]
  pub user_id: String,
}

pub fn create_tag(tag: CreateTag) -> Result<Tag, APIError> {
  match tag.validate() {
    Ok(_) => (),
    Err(_) => return Err(APIError::BadRequest),
  }

  let user = get_user(&tag.user_id);

  if user.is_err() {
    return Err(APIError::UserNotFound);
  }

  let category = get_category(&tag.category_id, &tag.user_id);

  if category.is_err() {
    return Err(APIError::CategoryNotFound);
  }

  let color_value = Color::from(tag.color.as_str());

  let mut conn = match establish_connection() {
    Ok(connection) => connection,
    Err(_) => return Err(APIError::DatabaseError),
  };

  let tag = Tag {
    id: Uuid::new_v4().to_string(),
    name: tag.name,
    color: color_value.to_string(),
    user_id: tag.user_id,
    category_id: tag.category_id,
    created_at: util::unix_ms(),
  };

  let tag_result = diesel::insert_into(tags::table)
    .values(&tag)
    .execute(&mut conn);

  match tag_result {
    Ok(_) => Ok(tag),
    _ => Err(APIError::DatabaseError),
  }
}

pub fn edit_tag(tag: EditTag) -> Result<Tag, APIError> {
  match tag.validate() {
    Ok(_) => (),
    Err(_) => return Err(APIError::BadRequest),
  }

  let user = get_user(&tag.user_id);

  if user.is_err() {
    return Err(APIError::UserNotFound);
  }

  let color_value = Color::from(tag.color.as_str());

  let mut conn = match establish_connection() {
    Ok(connection) => connection,
    Err(_) => return Err(APIError::DatabaseError),
  };

  let result = diesel::update(
    tags::table
      .filter(tags::id.eq(&tag.id))
      .filter(tags::user_id.eq(&tag.user_id)),
  )
  .set((
    tags::name.eq(&tag.name),
    tags::color.eq(color_value.to_string()),
  ))
  .execute(&mut conn);

  match result {
    Ok(_) => get_tag(&tag.id, &tag.user_id),
    Err(_) => Err(APIError::DatabaseError),
  }
}

pub fn get_tag(tag_id: &str, user_id: &str) -> Result<Tag, APIError> {
  let user = get_user(user_id);

  if user.is_err() {
    return Err(APIError::UserNotFound);
  }

  let mut conn = match establish_connection() {
    Ok(connection) => connection,
    Err(_) => return Err(APIError::DatabaseError),
  };

  let result = tags::table
    .filter(tags::id.eq(tag_id))
    .filter(tags::user_id.eq(user_id))
    .first::<Tag>(&mut conn);

  match result {
    Ok(tag) => Ok(tag),
    Err(_) => Err(APIError::TagNotFound),
  }
}

pub fn get_tags(tag_ids: Vec<&str>, user_id: &str) -> Result<Vec<Tag>, APIError> {
  let user = get_user(user_id);

  if user.is_err() {
    return Err(APIError::UserNotFound);
  }

  let mut conn = match establish_connection() {
    Ok(connection) => connection,
    Err(_) => return Err(APIError::DatabaseError),
  };

  let result = tags::table
    .filter(tags::id.eq_any(tag_ids))
    .filter(tags::user_id.eq(user_id))
    .order(tags::name.asc())
    .load::<Tag>(&mut conn);

  match result {
    Ok(tags) => Ok(tags),
    Err(_) => Err(APIError::DatabaseError),
  }
}

pub fn delete_tag(tag_id: &str, user_id: &str) -> Result<bool, APIError> {
  let user = get_user(user_id);

  if user.is_err() {
    return Err(APIError::UserNotFound);
  }

  let mut conn = match establish_connection() {
    Ok(connection) => connection,
    Err(_) => return Err(APIError::DatabaseError),
  };

  let result = diesel::delete(
    tags::table
      .filter(tags::id.eq(tag_id))
      .filter(tags::user_id.eq(user_id)),
  )
  .execute(&mut conn);

  match result {
    Ok(count) => Ok(count > 0),
    Err(_) => Err(APIError::DatabaseError),
  }
}

pub fn delete_all_category_tags(category_id: &str, user_id: &str) -> Result<bool, APIError> {
  let user = get_user(user_id);

  if user.is_err() {
    return Err(APIError::UserNotFound);
  }

  let category = get_category(category_id, user_id);

  if category.is_err() {
    return Err(APIError::CategoryNotFound);
  }

  let category_tags = get_category_tags(category_id, user_id)?;

  for tag in category_tags {
    delete_tag(&tag.id, user_id)?;
  }

  Ok(true)
}

pub fn get_category_tags(category_id: &str, user_id: &str) -> Result<Vec<Tag>, APIError> {
  let user = get_user(user_id);

  if user.is_err() {
    return Err(APIError::UserNotFound);
  }

  let category = get_category(category_id, user_id);

  if category.is_err() {
    return Err(APIError::CategoryNotFound);
  }

  let mut conn = match establish_connection() {
    Ok(connection) => connection,
    Err(_) => return Err(APIError::DatabaseError),
  };

  let result = tags::table
    .filter(tags::category_id.eq(category_id))
    .filter(tags::user_id.eq(user_id))
    .order(tags::name.asc())
    .load::<Tag>(&mut conn);

  match result {
    Ok(tags) => Ok(tags),
    Err(_) => Err(APIError::DatabaseError),
  }
}

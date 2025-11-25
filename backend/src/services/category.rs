use crate::{
  establish_connection,
  schema::categories,
  services::{
    get_user,
    tag::{delete_all_category_tags, get_category_tags, Tag},
  },
  util::{self, EphemerideError},
};
use diesel::{
  prelude::{Insertable, Queryable},
  ExpressionMethods, QueryDsl, RunQueryDsl,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Insertable, Queryable)]
#[diesel(table_name = categories)]
pub struct Category {
  pub id: String,
  pub name: String,
  pub user_id: String,
  pub created_at: i64,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateCategory {
  #[validate(length(min = 1, max = 255))]
  pub name: String,
  #[validate(length(min = 1, max = 255))]
  pub user_id: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct EditCategory {
  #[validate(length(min = 1, max = 255))]
  pub id: String,
  #[validate(length(min = 1, max = 255))]
  pub name: String,
  #[validate(length(min = 1, max = 255))]
  pub user_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CategoryWithTags {
  pub id: String,
  pub name: String,
  pub user_id: String,
  pub created_at: i64,
  pub tags: Vec<Tag>,
}

pub fn create_category(category: CreateCategory) -> Result<Category, EphemerideError> {
  match category.validate() {
    Ok(_) => (),
    Err(_) => return Err(EphemerideError::BadRequest),
  }

  let user = get_user(&category.user_id);

  if user.is_err() {
    return Err(EphemerideError::UserNotFound);
  }

  let mut conn = establish_connection();

  let new_category = Category {
    id: Uuid::new_v4().to_string(),
    name: category.name,
    user_id: category.user_id,
    created_at: util::unix_ms(),
  };

  let result = diesel::insert_into(categories::table)
    .values(&new_category)
    .execute(&mut conn);

  match result {
    Ok(_) => Ok(new_category),
    Err(_) => Err(EphemerideError::DatabaseError),
  }
}

pub fn edit_category(category: EditCategory) -> Result<Category, EphemerideError> {
  match category.validate() {
    Ok(_) => (),
    Err(_) => return Err(EphemerideError::BadRequest),
  }

  let user = get_user(&category.user_id);

  if user.is_err() {
    return Err(EphemerideError::UserNotFound);
  }

  let mut conn = establish_connection();

  let result = diesel::update(
    categories::table
      .filter(categories::id.eq(&category.id))
      .filter(categories::user_id.eq(&category.user_id)),
  )
  .set(categories::name.eq(&category.name))
  .execute(&mut conn);

  match result {
    Ok(_) => get_category(&category.id, &category.user_id),
    Err(_) => Err(EphemerideError::DatabaseError),
  }
}

pub fn get_category(category_id: &str, user_id: &str) -> Result<Category, EphemerideError> {
  let mut conn = establish_connection();

  let result = categories::table
    .filter(categories::id.eq(category_id))
    .filter(categories::user_id.eq(user_id))
    .first::<Category>(&mut conn);

  match result {
    Ok(category) => Ok(category),
    Err(_) => Err(EphemerideError::CategoryNotFound),
  }
}

pub fn get_category_with_tags(
  category_id: &str,
  user_id: &str,
) -> Result<CategoryWithTags, EphemerideError> {
  let category = match get_category(category_id, user_id) {
    Ok(category) => category,
    Err(_) => return Err(EphemerideError::CategoryNotFound),
  };

  let tags = match get_category_tags(category_id, user_id) {
    Ok(tags) => tags,
    Err(_) => return Err(EphemerideError::DatabaseError),
  };

  let category_with_tags = CategoryWithTags {
    id: category.id,
    name: category.name,
    user_id: category.user_id,
    created_at: category.created_at,
    tags,
  };

  Ok(category_with_tags)
}

pub fn get_user_categories_with_tags(
  user_id: &str,
) -> Result<Vec<CategoryWithTags>, EphemerideError> {
  let categories = match get_all_categories(user_id) {
    Ok(categories) => categories,
    Err(_) => return Err(EphemerideError::DatabaseError),
  };

  let mut categories_with_tags: Vec<CategoryWithTags> = Vec::new();

  for category in categories {
    let tags = match get_category_tags(&category.id, user_id) {
      Ok(tags) => tags,
      Err(_) => return Err(EphemerideError::DatabaseError),
    };

    let category_with_tags = CategoryWithTags {
      id: category.id,
      name: category.name,
      user_id: category.user_id,
      created_at: category.created_at,
      tags,
    };

    categories_with_tags.push(category_with_tags);
  }

  Ok(categories_with_tags)
}

pub fn delete_category(category_id: &str, user_id: &str) -> Result<bool, EphemerideError> {
  let user = get_user(user_id);

  if user.is_err() {
    return Err(EphemerideError::UserNotFound);
  }

  let category = get_category(category_id, user_id);

  if category.is_err() {
    return Err(EphemerideError::CategoryNotFound);
  }

  let mut conn = establish_connection();

  let deleted_tags = delete_all_category_tags(category_id, user_id);

  if deleted_tags.is_err() {
    return Err(EphemerideError::DatabaseError);
  }

  let result = diesel::delete(
    categories::table
      .filter(categories::id.eq(category_id))
      .filter(categories::user_id.eq(user_id)),
  )
  .execute(&mut conn);

  match result {
    Ok(count) => Ok(count > 0),
    Err(_) => Err(EphemerideError::DatabaseError),
  }
}

pub fn get_all_categories(user_id: &str) -> Result<Vec<Category>, EphemerideError> {
  let user = get_user(user_id);

  if user.is_err() {
    return Err(EphemerideError::UserNotFound);
  }

  let mut conn = establish_connection();

  let result = categories::table
    .filter(categories::user_id.eq(user_id))
    .order(categories::name.asc())
    .load::<Category>(&mut conn);

  match result {
    Ok(categories) => Ok(categories),
    Err(_) => Err(EphemerideError::DatabaseError),
  }
}

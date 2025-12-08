use crate::{
  establish_connection,
  schema::{categories, entries, tags},
  services::{
    category::{create_category, CreateCategory},
    tag::{create_tag, CreateTag},
    user::get_user,
  },
  util::error::APIError,
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

pub fn create_default_data(user_id: String) -> Result<bool, APIError> {
  let default_categories = vec!["Activities", "Tags"];

  let default_tags = vec![
    ("Activities", "Work", "base"),
    ("Activities", "Movie", "base"),
    ("Activities", "Exercise", "base"),
    ("Activities", "Read", "base"),
    ("Activities", "Shopping", "base"),
    ("Activities", "Gaming", "base"),
    ("Tags", "Travel", "base"),
    ("Tags", "Important", "blue"),
    ("Tags", "Sick", "red"),
  ];

  for category_name in default_categories {
    let category_result = create_category(CreateCategory {
      name: category_name.to_string(),
      user_id: user_id.clone(),
    });

    let category = match category_result {
      Ok(category) => category,
      Err(_) => return Err(APIError::DatabaseError),
    };

    for (cat_name, tag_name, color) in &default_tags {
      if *cat_name == category_name {
        let tag_result = create_tag(CreateTag {
          name: tag_name.to_string(),
          color: color.to_string(),
          category_id: category.id.clone(),
          user_id: user_id.clone(),
        });

        if tag_result.is_err() {
          return Err(APIError::DatabaseError);
        }
      }
    }
  }

  Ok(true)
}

pub fn delete_all_user_data(user_id: &str) -> Result<bool, APIError> {
  let user = get_user(user_id);

  if user.is_err() {
    return Err(APIError::UserNotFound);
  }

  let mut conn = match establish_connection() {
    Ok(connection) => connection,
    Err(_) => return Err(APIError::DatabaseError),
  };

  let delete_entries =
    diesel::delete(entries::table.filter(entries::user_id.eq(user_id))).execute(&mut conn);

  if delete_entries.is_err() {
    return Err(APIError::DatabaseError);
  }

  let delete_tags =
    diesel::delete(tags::table.filter(tags::user_id.eq(user_id))).execute(&mut conn);

  if delete_tags.is_err() {
    return Err(APIError::DatabaseError);
  }

  let delete_categories =
    diesel::delete(categories::table.filter(categories::user_id.eq(user_id))).execute(&mut conn);

  if delete_categories.is_err() {
    return Err(APIError::DatabaseError);
  }

  Ok(true)
}

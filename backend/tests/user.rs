use diarycomputer::services::{auth, log, user};
use uuid::Uuid;

#[test]
fn create_user() {
  let random_name = Uuid::new_v4().to_string();
  let email = format!("{random_name}@example.com");

  let user = user::CreateUser {
    name: random_name.clone(),
    email: email.clone(),
    password: "password".to_string(),
    invite: None,
  };

  let created_user = user::create_user(user);

  assert!(created_user.is_ok());

  let found_user = user::get_user(&created_user.unwrap().id);

  assert!(found_user.is_ok());
  let found_user = found_user.unwrap();
  assert_eq!(found_user.name, random_name);

  let found_categories = log::get_all_categories(&found_user.id);

  assert!(found_categories.is_ok());
  let categories = found_categories.unwrap();
  assert!(!categories.is_empty());

  for category in categories {
    let found_tags = log::get_category_tags(&category.id, &found_user.id);
    assert!(found_tags.is_ok());
    assert!(!found_tags.unwrap().is_empty());
  }
}

#[test]
fn create_user_and_delete() {
  let random_name = Uuid::new_v4().to_string();
  let email = format!("{random_name}@example.com");

  let user = user::CreateUser {
    name: random_name.clone(),
    email: email.clone(),
    password: "password".to_string(),
    invite: None,
  };

  let created_user = user::create_user(user);

  assert!(created_user.is_ok());

  let found_user = user::get_user(&created_user.unwrap().id);

  assert!(found_user.is_ok());

  let deleted = user::delete_user(&found_user.unwrap().id).unwrap_or_default();

  assert!(deleted);

  let found_user = user::get_user_id(&email);

  assert!(found_user.is_err());
}

#[test]
fn delete_user_that_does_not_exist() {
  let deleted = user::delete_user("INVALID_ID").unwrap_or_default();

  assert!(!deleted);
}

#[test]
fn updates_user() {
  let random_name = Uuid::new_v4().to_string();
  let email = format!("{random_name}@example.com");

  let user = user::CreateUser {
    name: random_name.clone(),
    email: email.clone(),
    password: "password".to_string(),
    invite: None,
  };

  let created_user = user::create_user(user);

  assert!(created_user.is_ok());

  let found_user = user::get_user(&created_user.unwrap().id);

  assert!(found_user.is_ok());
  let found_user = found_user.unwrap();
  let found_user_name = found_user.name.clone();
  assert_eq!(found_user_name, random_name);

  let new_random_name = Uuid::new_v4().to_string();
  let new_email = format!("{new_random_name}@example.com");

  let updated_user = user::UpdateUser {
    name: new_random_name.clone(),
    email: new_email.clone(),
  };

  let updated = user::update_user(&found_user.id, updated_user);

  assert!(updated.is_ok());

  let found_user = user::get_user(&found_user.id);

  assert!(found_user.is_ok());
  assert_eq!(found_user.unwrap().name, new_random_name);
}

#[test]
fn creates_a_session() {
  let random_name = Uuid::new_v4().to_string();
  let email = format!("{random_name}@example.com");
  let password = "password".to_string();

  let user = user::CreateUser {
    name: random_name.clone(),
    email: email.clone(),
    password: password.clone(),
    invite: None,
  };

  let created_user = user::create_user(user);

  assert!(created_user.is_ok());

  let user_id = user::get_user_id(&email);

  assert!(user_id.is_ok());

  let credentials = auth::UserCredentials {
    email: email.clone(),
    password: password.clone(),
  };
  let metadata = auth::SessionMetadata {
    ip_address: "SYSTEM".to_string(),
    user_agent: "SYSTEM".to_string(),
  };

  let session = auth::create_user_session(credentials, metadata);

  assert!(session.is_ok());

  let found_session = auth::get_user_session_by_id(&session.unwrap().id);

  assert!(found_session.is_ok());
}

#[test]
fn deletes_a_session() {
  let random_name = Uuid::new_v4().to_string();
  let email = format!("{random_name}@example.com");
  let password = "password".to_string();

  let user = user::CreateUser {
    name: random_name.clone(),
    email: email.clone(),
    password: password.clone(),
    invite: None,
  };

  let created_user = user::create_user(user);

  assert!(created_user.is_ok());

  let found_user = user::get_user_id(&email);

  assert!(found_user.is_ok());

  let credentials = auth::UserCredentials {
    email: email.clone(),
    password: password.clone(),
  };
  let metadata = auth::SessionMetadata {
    ip_address: "SYSTEM".to_string(),
    user_agent: "SYSTEM".to_string(),
  };

  let session = auth::create_user_session(credentials, metadata);

  assert!(session.is_ok());

  let session_id = session.unwrap().id;
  let found_session = auth::get_user_session_by_id(&session_id);

  assert!(found_session.is_ok());

  let deleted = auth::delete_user_session(&session_id);

  assert!(deleted.unwrap());

  let found_session = auth::get_user_session_by_id(&session_id);

  assert!(found_session.is_err());
}

#[test]
fn user_count() {
  let _created_user = user::create_user(user::CreateUser {
    name: Uuid::new_v4().to_string(),
    email: format!("{}@example.com", Uuid::new_v4()),
    password: "password".to_string(),
    invite: None,
  });

  let count = user::user_count();
  let active_count = user::active_user_count(0);
  let active_count_max_i64 = user::active_user_count(i64::MAX);

  assert!(count.is_ok());
  assert!(count.unwrap() > 0);
  assert!(active_count.is_ok());
  assert!(active_count.unwrap() > 0);
  assert!(active_count_max_i64.is_ok());
  assert!(active_count_max_i64.unwrap() == 0);
}

#[test]
fn too_long_password() {
  let random_name = Uuid::new_v4().to_string();
  let email = format!("{random_name}@example.com");
  let long_password = "p".repeat(73); // 73 characters, exceeding the 72 character limit

  let user = user::CreateUser {
    name: random_name.clone(),
    email: email.clone(),
    password: long_password,
    invite: None,
  };

  let created_user = user::create_user(user);

  assert!(created_user.is_err());
}

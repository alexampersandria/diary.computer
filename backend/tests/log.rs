use diary_dot_computer_backend::{
  services::{log, user},
  util::APIError,
};
use uuid::Uuid;

fn create_user() -> user::UserDetails {
  let random_name = Uuid::new_v4().to_string();
  let email = format!("{random_name}@example.com");

  let user_data = user::CreateUser {
    name: random_name.clone(),
    email: email.clone(),
    password: "password".to_string(),
    invite: None,
  };

  user::create_user(user_data).expect("Failed to create test user")
}

#[test]
fn create_category() {
  let user = create_user();

  let category = log::create_category(log::CreateCategory {
    name: "Test Category".to_string(),
    user_id: user.id.clone(),
  });

  assert!(category.is_ok());
  let category = category.unwrap();
  assert_eq!(category.name, "Test Category");
  assert_eq!(category.user_id, user.id);
}

#[test]
fn edit_category() {
  let user = create_user();

  let category = log::create_category(log::CreateCategory {
    name: "Original Name".to_string(),
    user_id: user.id.clone(),
  })
  .unwrap();

  let edited = log::edit_category(log::EditCategory {
    id: category.id.clone(),
    name: "Updated Name".to_string(),
    user_id: user.id.clone(),
  });

  assert!(edited.is_ok());
  let edited = edited.unwrap();
  assert_eq!(edited.name, "Updated Name");
  assert_eq!(edited.id, category.id);
}

#[test]
fn get_category() {
  let user = create_user();

  let category = log::create_category(log::CreateCategory {
    name: "Get Test".to_string(),
    user_id: user.id.clone(),
  })
  .unwrap();

  let found = log::get_category(&category.id, &user.id);

  assert!(found.is_ok());
  let found = found.unwrap();
  assert_eq!(found.id, category.id);
  assert_eq!(found.name, "Get Test");
}

#[test]
fn get_all_categories() {
  let user = create_user();

  log::create_category(log::CreateCategory {
    name: "Category 1".to_string(),
    user_id: user.id.clone(),
  })
  .unwrap();
  log::create_category(log::CreateCategory {
    name: "Category 2".to_string(),
    user_id: user.id.clone(),
  })
  .unwrap();

  let categories = log::get_all_categories(&user.id);

  assert!(categories.is_ok());
  let categories = categories.unwrap();
  assert!(categories.len() >= 2);
}

#[test]
fn get_category_with_tags() {
  let user = create_user();

  let category = log::create_category(log::CreateCategory {
    name: "Category with Tags".to_string(),
    user_id: user.id.clone(),
  })
  .unwrap();
  log::create_tag(log::CreateTag {
    name: "Tag 1".to_string(),
    color: "blue".to_string(),
    category_id: category.id.clone(),
    user_id: user.id.clone(),
  })
  .unwrap();
  log::create_tag(log::CreateTag {
    name: "Tag 2".to_string(),
    color: "red".to_string(),
    category_id: category.id.clone(),
    user_id: user.id.clone(),
  })
  .unwrap();

  let category_with_tags = log::get_category_with_tags(&category.id, &user.id);

  assert!(category_with_tags.is_ok());
  let category_with_tags = category_with_tags.unwrap();
  assert_eq!(category_with_tags.id, category.id);
  assert_eq!(category_with_tags.tags.len(), 2);
}

#[test]
fn get_category_with_tags_not_found() {
  let user = create_user();

  let result = log::get_category_with_tags("nonexistent_id", &user.id);

  assert!(result.is_err());
  assert_eq!(result.err().unwrap(), APIError::CategoryNotFound);
}

#[test]
fn get_user_categories_with_tags() {
  let user = create_user();

  let cat1 = log::create_category(log::CreateCategory {
    name: "Category 1".to_string(),
    user_id: user.id.clone(),
  })
  .unwrap();
  let cat2 = log::create_category(log::CreateCategory {
    name: "Category 2".to_string(),
    user_id: user.id.clone(),
  })
  .unwrap();

  log::create_tag(log::CreateTag {
    name: "Tag 1".to_string(),
    color: "blue".to_string(),
    category_id: cat1.id.clone(),
    user_id: user.id.clone(),
  })
  .unwrap();
  log::create_tag(log::CreateTag {
    name: "Tag 2".to_string(),
    color: "red".to_string(),
    category_id: cat2.id.clone(),
    user_id: user.id.clone(),
  })
  .unwrap();

  let categories_with_tags = log::get_user_categories_with_tags(&user.id);

  assert!(categories_with_tags.is_ok());
  let categories_with_tags = categories_with_tags.unwrap();
  assert!(categories_with_tags.len() >= 2);

  for category in &categories_with_tags {
    if category.id == cat1.id || category.id == cat2.id {
      assert!(!category.tags.is_empty());
    }
  }
}

#[test]
fn delete_category() {
  let user = create_user();

  let category = log::create_category(log::CreateCategory {
    name: "To Delete".to_string(),
    user_id: user.id.clone(),
  })
  .unwrap();
  log::create_tag(log::CreateTag {
    name: "Tag".to_string(),
    color: "blue".to_string(),
    category_id: category.id.clone(),
    user_id: user.id.clone(),
  })
  .unwrap();

  let deleted = log::delete_category(&category.id, &user.id);

  assert!(deleted.is_ok());
  assert!(deleted.unwrap());

  let found = log::get_category(&category.id, &user.id);
  assert!(found.is_err());
}

#[test]
fn create_tag() {
  let user = create_user();
  let category = log::create_category(log::CreateCategory {
    name: "Test Category".to_string(),
    user_id: user.id.clone(),
  })
  .unwrap();

  let tag = log::create_tag(log::CreateTag {
    name: "Test Tag".to_string(),
    color: "blue".to_string(),
    category_id: category.id.clone(),
    user_id: user.id.clone(),
  });

  assert!(tag.is_ok());
  let tag = tag.unwrap();
  assert_eq!(tag.name, "Test Tag");
  assert_eq!(tag.user_id, user.id);
  assert_eq!(tag.category_id, category.id);
}

#[test]
fn edit_tag() {
  let user = create_user();
  let category = log::create_category(log::CreateCategory {
    name: "Test Category".to_string(),
    user_id: user.id.clone(),
  })
  .unwrap();
  let tag = log::create_tag(log::CreateTag {
    name: "Original".to_string(),
    color: "blue".to_string(),
    category_id: category.id.clone(),
    user_id: user.id.clone(),
  })
  .unwrap();

  let edited = log::edit_tag(log::EditTag {
    id: tag.id.clone(),
    name: "Updated".to_string(),
    color: "red".to_string(),
    user_id: user.id.clone(),
  });

  assert!(edited.is_ok());
  let edited = edited.unwrap();
  assert_eq!(edited.name, "Updated");
  assert_eq!(edited.color, "red");
}

#[test]
fn get_tag() {
  let user = create_user();
  let category = log::create_category(log::CreateCategory {
    name: "Test Category".to_string(),
    user_id: user.id.clone(),
  })
  .unwrap();
  let tag = log::create_tag(log::CreateTag {
    name: "Test Tag".to_string(),
    color: "blue".to_string(),
    category_id: category.id.clone(),
    user_id: user.id.clone(),
  })
  .unwrap();

  let found = log::get_tag(&tag.id, &user.id);

  assert!(found.is_ok());
  let found = found.unwrap();
  assert_eq!(found.id, tag.id);
  assert_eq!(found.name, "Test Tag");
}

#[test]
fn get_tags() {
  let user = create_user();
  let category = log::create_category(log::CreateCategory {
    name: "Test Category".to_string(),
    user_id: user.id.clone(),
  })
  .unwrap();
  let tag1 = log::create_tag(log::CreateTag {
    name: "Tag 1".to_string(),
    color: "blue".to_string(),
    category_id: category.id.clone(),
    user_id: user.id.clone(),
  })
  .unwrap();
  let tag2 = log::create_tag(log::CreateTag {
    name: "Tag 2".to_string(),
    color: "red".to_string(),
    category_id: category.id.clone(),
    user_id: user.id.clone(),
  })
  .unwrap();

  let tag_ids = vec![tag1.id.as_str(), tag2.id.as_str()];
  let tags = log::get_tags(tag_ids, &user.id);

  assert!(tags.is_ok());
  let tags = tags.unwrap();
  assert_eq!(tags.len(), 2);
}

#[test]
fn get_category_tags() {
  let user = create_user();
  let category = log::create_category(log::CreateCategory {
    name: "Test Category".to_string(),
    user_id: user.id.clone(),
  })
  .unwrap();
  log::create_tag(log::CreateTag {
    name: "Tag 1".to_string(),
    color: "blue".to_string(),
    category_id: category.id.clone(),
    user_id: user.id.clone(),
  })
  .unwrap();
  log::create_tag(log::CreateTag {
    name: "Tag 2".to_string(),
    color: "red".to_string(),
    category_id: category.id.clone(),
    user_id: user.id.clone(),
  })
  .unwrap();

  let tags = log::get_category_tags(&category.id, &user.id);

  assert!(tags.is_ok());
  let tags = tags.unwrap();
  assert_eq!(tags.len(), 2);
}

#[test]
fn delete_tag() {
  let user = create_user();
  let category = log::create_category(log::CreateCategory {
    name: "Test Category".to_string(),
    user_id: user.id.clone(),
  })
  .unwrap();
  let tag = log::create_tag(log::CreateTag {
    name: "To Delete".to_string(),
    color: "blue".to_string(),
    category_id: category.id.clone(),
    user_id: user.id.clone(),
  })
  .unwrap();

  let deleted = log::delete_tag(&tag.id, &user.id);

  assert!(deleted.is_ok());
  assert!(deleted.unwrap());

  let found = log::get_tag(&tag.id, &user.id);
  assert!(found.is_err());
}

#[test]
fn delete_all_category_tags() {
  let user = create_user();
  let category = log::create_category(log::CreateCategory {
    name: "Test Category".to_string(),
    user_id: user.id.clone(),
  })
  .unwrap();
  log::create_tag(log::CreateTag {
    name: "Tag 1".to_string(),
    color: "blue".to_string(),
    category_id: category.id.clone(),
    user_id: user.id.clone(),
  })
  .unwrap();
  log::create_tag(log::CreateTag {
    name: "Tag 2".to_string(),
    color: "red".to_string(),
    category_id: category.id.clone(),
    user_id: user.id.clone(),
  })
  .unwrap();

  let deleted = log::delete_all_category_tags(&category.id, &user.id);

  assert!(deleted.is_ok());
  assert!(deleted.unwrap());

  let tags = log::get_category_tags(&category.id, &user.id);
  assert!(tags.is_ok());
  assert_eq!(tags.unwrap().len(), 0);
}

#[test]
fn create_entry() {
  let user = create_user();
  let category = log::create_category(log::CreateCategory {
    name: "Test Category".to_string(),
    user_id: user.id.clone(),
  })
  .unwrap();
  let tag = log::create_tag(log::CreateTag {
    name: "Test Tag".to_string(),
    color: "blue".to_string(),
    category_id: category.id.clone(),
    user_id: user.id.clone(),
  })
  .unwrap();

  let entry = log::create_entry(log::CreateEntry {
    date: "2025-10-17".to_string(),
    mood: 5,
    entry: Some("Test entry content".to_string()),
    selected_tags: vec![tag.id.clone()],
    user_id: user.id.clone(),
  });

  assert!(entry.is_ok());
  let entry = entry.unwrap();
  assert_eq!(
    entry.date,
    chrono::NaiveDate::from_ymd_opt(2025, 10, 17).unwrap()
  );
  assert_eq!(entry.mood, 5);
  assert_eq!(entry.entry, Some("Test entry content".to_string()));
  assert_eq!(entry.selected_tags.len(), 1);
  assert_eq!(entry.selected_tags[0], tag.id);
}

#[test]
fn create_entry_without_content() {
  let user = create_user();

  let entry = log::create_entry(log::CreateEntry {
    date: "2025-10-17".to_string(),
    mood: 3,
    entry: None,
    selected_tags: vec![],
    user_id: user.id.clone(),
  });

  assert!(entry.is_ok());
  let entry = entry.unwrap();
  assert_eq!(entry.entry, None);
  assert_eq!(entry.selected_tags.len(), 0);
}

#[test]
fn edit_entry() {
  let user = create_user();
  let category = log::create_category(log::CreateCategory {
    name: "Test Category".to_string(),
    user_id: user.id.clone(),
  })
  .unwrap();
  let tag1 = log::create_tag(log::CreateTag {
    name: "Tag 1".to_string(),
    color: "blue".to_string(),
    category_id: category.id.clone(),
    user_id: user.id.clone(),
  })
  .unwrap();
  let tag2 = log::create_tag(log::CreateTag {
    name: "Tag 2".to_string(),
    color: "red".to_string(),
    category_id: category.id.clone(),
    user_id: user.id.clone(),
  })
  .unwrap();

  let entry = log::create_entry(log::CreateEntry {
    date: "2025-10-17".to_string(),
    mood: 3,
    entry: Some("Original content".to_string()),
    selected_tags: vec![tag1.id.clone()],
    user_id: user.id.clone(),
  })
  .unwrap();

  let edited = log::edit_entry(log::EditEntry {
    id: entry.id.clone(),
    date: "2025-10-18".to_string(),
    mood: 4,
    entry: Some("Updated content".to_string()),
    selected_tags: vec![tag2.id.clone()],
    user_id: user.id.clone(),
  });

  assert!(edited.is_ok());
  let edited = edited.unwrap();
  assert_eq!(
    edited.date,
    chrono::NaiveDate::from_ymd_opt(2025, 10, 18).unwrap()
  );
  assert_eq!(edited.mood, 4);
  assert_eq!(edited.entry, Some("Updated content".to_string()));
  assert_eq!(edited.selected_tags.len(), 1);
  assert_eq!(edited.selected_tags[0], tag2.id);
}

#[test]
fn get_entry_with_tags() {
  let user = create_user();
  let category = log::create_category(log::CreateCategory {
    name: "Test Category".to_string(),
    user_id: user.id.clone(),
  })
  .unwrap();
  let tag = log::create_tag(log::CreateTag {
    name: "Test Tag".to_string(),
    color: "blue".to_string(),
    category_id: category.id.clone(),
    user_id: user.id.clone(),
  })
  .unwrap();

  let entry = log::create_entry(log::CreateEntry {
    date: "2025-10-17".to_string(),
    mood: 5,
    entry: Some("Test entry".to_string()),
    selected_tags: vec![tag.id.clone()],
    user_id: user.id.clone(),
  })
  .unwrap();

  let found = log::get_entry_with_tags(&entry.id, &user.id);

  assert!(found.is_ok());
  let found = found.unwrap();
  assert_eq!(found.id, entry.id);
  assert_eq!(found.selected_tags.len(), 1);
  assert_eq!(found.selected_tags[0], tag.id);
}

#[test]
fn create_default_data() {
  let user = create_user();

  let result = log::create_default_data(user.id.clone());

  assert!(result.is_ok());

  let categories = log::get_all_categories(&user.id).unwrap();
  assert!(categories.len() >= 2);

  let activities_cat = categories.iter().find(|c| c.name == "Activities");
  let tags_cat = categories.iter().find(|c| c.name == "Tags");

  assert!(activities_cat.is_some());
  assert!(tags_cat.is_some());

  if let Some(activities) = activities_cat {
    let tags = log::get_category_tags(&activities.id, &user.id).unwrap();
    assert!(tags.len() >= 6);
  }

  if let Some(tags) = tags_cat {
    let tag_list = log::get_category_tags(&tags.id, &user.id).unwrap();
    assert!(tag_list.len() >= 3);
  }
}

// Validation tests

#[test]
fn create_category_empty_name() {
  let user = create_user();

  let category = log::create_category(log::CreateCategory {
    name: "".to_string(),
    user_id: user.id.clone(),
  });

  assert!(category.is_err());
}

#[test]
fn create_category_name_too_long() {
  let user = create_user();

  let long_name = "a".repeat(256);
  let category = log::create_category(log::CreateCategory {
    name: long_name,
    user_id: user.id.clone(),
  });

  assert!(category.is_err());
}

#[test]
fn edit_category_empty_name() {
  let user = create_user();
  let category = log::create_category(log::CreateCategory {
    name: "Original Name".to_string(),
    user_id: user.id.clone(),
  })
  .unwrap();

  let edited = log::edit_category(log::EditCategory {
    id: category.id.clone(),
    name: "".to_string(),
    user_id: user.id.clone(),
  });

  assert!(edited.is_err());
}

#[test]
fn edit_category_name_too_long() {
  let user = create_user();
  let category = log::create_category(log::CreateCategory {
    name: "Original Name".to_string(),
    user_id: user.id.clone(),
  })
  .unwrap();

  let long_name = "a".repeat(256);
  let edited = log::edit_category(log::EditCategory {
    id: category.id.clone(),
    name: long_name,
    user_id: user.id.clone(),
  });

  assert!(edited.is_err());
}

#[test]
fn create_tag_empty_name() {
  let user = create_user();
  let category = log::create_category(log::CreateCategory {
    name: "Test Category".to_string(),
    user_id: user.id.clone(),
  })
  .unwrap();

  let tag = log::create_tag(log::CreateTag {
    name: "".to_string(),
    color: "blue".to_string(),
    category_id: category.id.clone(),
    user_id: user.id.clone(),
  });

  assert!(tag.is_err());
}

#[test]
fn create_tag_name_too_long() {
  let user = create_user();
  let category = log::create_category(log::CreateCategory {
    name: "Test Category".to_string(),
    user_id: user.id.clone(),
  })
  .unwrap();

  let long_name = "a".repeat(256);
  let tag = log::create_tag(log::CreateTag {
    name: long_name,
    color: "blue".to_string(),
    category_id: category.id.clone(),
    user_id: user.id.clone(),
  });

  assert!(tag.is_err());
}

#[test]
fn create_tag_empty_color() {
  let user = create_user();
  let category = log::create_category(log::CreateCategory {
    name: "Test Category".to_string(),
    user_id: user.id.clone(),
  })
  .unwrap();

  let tag = log::create_tag(log::CreateTag {
    name: "Test Tag".to_string(),
    color: "".to_string(),
    category_id: category.id.clone(),
    user_id: user.id.clone(),
  });

  assert!(tag.is_err());
}

#[test]
fn create_tag_color_too_long() {
  let user = create_user();
  let category = log::create_category(log::CreateCategory {
    name: "Test Category".to_string(),
    user_id: user.id.clone(),
  })
  .unwrap();

  let long_color = "a".repeat(17);
  let tag = log::create_tag(log::CreateTag {
    name: "Test Tag".to_string(),
    color: long_color,
    category_id: category.id.clone(),
    user_id: user.id.clone(),
  });

  assert!(tag.is_err());
}

#[test]
fn edit_tag_empty_name() {
  let user = create_user();
  let category = log::create_category(log::CreateCategory {
    name: "Test Category".to_string(),
    user_id: user.id.clone(),
  })
  .unwrap();
  let tag = log::create_tag(log::CreateTag {
    name: "Original".to_string(),
    color: "blue".to_string(),
    category_id: category.id.clone(),
    user_id: user.id.clone(),
  })
  .unwrap();

  let edited = log::edit_tag(log::EditTag {
    id: tag.id.clone(),
    name: "".to_string(),
    color: "red".to_string(),
    user_id: user.id.clone(),
  });

  assert!(edited.is_err());
}

#[test]
fn edit_tag_name_too_long() {
  let user = create_user();
  let category = log::create_category(log::CreateCategory {
    name: "Test Category".to_string(),
    user_id: user.id.clone(),
  })
  .unwrap();
  let tag = log::create_tag(log::CreateTag {
    name: "Original".to_string(),
    color: "blue".to_string(),
    category_id: category.id.clone(),
    user_id: user.id.clone(),
  })
  .unwrap();

  let long_name = "a".repeat(256);
  let edited = log::edit_tag(log::EditTag {
    id: tag.id.clone(),
    name: long_name,
    color: "red".to_string(),
    user_id: user.id.clone(),
  });

  assert!(edited.is_err());
}

#[test]
fn edit_tag_empty_color() {
  let user = create_user();
  let category = log::create_category(log::CreateCategory {
    name: "Test Category".to_string(),
    user_id: user.id.clone(),
  })
  .unwrap();
  let tag = log::create_tag(log::CreateTag {
    name: "Original".to_string(),
    color: "blue".to_string(),
    category_id: category.id.clone(),
    user_id: user.id.clone(),
  })
  .unwrap();

  let edited = log::edit_tag(log::EditTag {
    id: tag.id.clone(),
    name: "Updated".to_string(),
    color: "".to_string(),
    user_id: user.id.clone(),
  });

  assert!(edited.is_err());
}

#[test]
fn edit_tag_color_too_long() {
  let user = create_user();
  let category = log::create_category(log::CreateCategory {
    name: "Test Category".to_string(),
    user_id: user.id.clone(),
  })
  .unwrap();
  let tag = log::create_tag(log::CreateTag {
    name: "Original".to_string(),
    color: "blue".to_string(),
    category_id: category.id.clone(),
    user_id: user.id.clone(),
  })
  .unwrap();

  let long_color = "a".repeat(17);
  let edited = log::edit_tag(log::EditTag {
    id: tag.id.clone(),
    name: "Updated".to_string(),
    color: long_color,
    user_id: user.id.clone(),
  });

  assert!(edited.is_err());
}

#[test]
fn create_entry_empty_date() {
  let user = create_user();

  let entry = log::create_entry(log::CreateEntry {
    date: "".to_string(),
    mood: 3,
    entry: Some("Test content".to_string()),
    selected_tags: vec![],
    user_id: user.id.clone(),
  });

  assert!(entry.is_err());
}

#[test]
fn create_entry_date_too_long() {
  let user = create_user();

  let long_date = "a".repeat(256);
  let entry = log::create_entry(log::CreateEntry {
    date: long_date,
    mood: 3,
    entry: Some("Test content".to_string()),
    selected_tags: vec![],
    user_id: user.id.clone(),
  });

  assert!(entry.is_err());
}

#[test]
fn create_entry_mood_too_low() {
  let user = create_user();

  let entry = log::create_entry(log::CreateEntry {
    date: "2025-10-17".to_string(),
    mood: 0,
    entry: Some("Test content".to_string()),
    selected_tags: vec![],
    user_id: user.id.clone(),
  });

  assert!(entry.is_err());
}

#[test]
fn create_entry_mood_too_high() {
  let user = create_user();

  let entry = log::create_entry(log::CreateEntry {
    date: "2025-10-17".to_string(),
    mood: 6,
    entry: Some("Test content".to_string()),
    selected_tags: vec![],
    user_id: user.id.clone(),
  });

  assert!(entry.is_err());
}

#[test]
fn create_entry_content_too_long() {
  let user = create_user();

  let long_content = "a".repeat(1001);
  let entry = log::create_entry(log::CreateEntry {
    date: "2025-10-17".to_string(),
    mood: 3,
    entry: Some(long_content),
    selected_tags: vec![],
    user_id: user.id.clone(),
  });

  assert!(entry.is_err());
}

#[test]
fn edit_entry_empty_date() {
  let user = create_user();
  let entry = log::create_entry(log::CreateEntry {
    date: "2025-10-17".to_string(),
    mood: 3,
    entry: Some("Original content".to_string()),
    selected_tags: vec![],
    user_id: user.id.clone(),
  })
  .unwrap();

  let edited = log::edit_entry(log::EditEntry {
    id: entry.id.clone(),
    date: "".to_string(),
    mood: 4,
    entry: Some("Updated content".to_string()),
    selected_tags: vec![],
    user_id: user.id.clone(),
  });

  assert!(edited.is_err());
}

#[test]
fn edit_entry_date_too_long() {
  let user = create_user();
  let entry = log::create_entry(log::CreateEntry {
    date: "2025-10-17".to_string(),
    mood: 3,
    entry: Some("Original content".to_string()),
    selected_tags: vec![],
    user_id: user.id.clone(),
  })
  .unwrap();

  let long_date = "a".repeat(256);
  let edited = log::edit_entry(log::EditEntry {
    id: entry.id.clone(),
    date: long_date,
    mood: 4,
    entry: Some("Updated content".to_string()),
    selected_tags: vec![],
    user_id: user.id.clone(),
  });

  assert!(edited.is_err());
}

#[test]
fn edit_entry_mood_too_low() {
  let user = create_user();
  let entry = log::create_entry(log::CreateEntry {
    date: "2025-10-17".to_string(),
    mood: 3,
    entry: Some("Original content".to_string()),
    selected_tags: vec![],
    user_id: user.id.clone(),
  })
  .unwrap();

  let edited = log::edit_entry(log::EditEntry {
    id: entry.id.clone(),
    date: "2025-10-18".to_string(),
    mood: 0,
    entry: Some("Updated content".to_string()),
    selected_tags: vec![],
    user_id: user.id.clone(),
  });

  assert!(edited.is_err());
}

#[test]
fn edit_entry_mood_too_high() {
  let user = create_user();
  let entry = log::create_entry(log::CreateEntry {
    date: "2025-10-17".to_string(),
    mood: 3,
    entry: Some("Original content".to_string()),
    selected_tags: vec![],
    user_id: user.id.clone(),
  })
  .unwrap();

  let edited = log::edit_entry(log::EditEntry {
    id: entry.id.clone(),
    date: "2025-10-18".to_string(),
    mood: 6,
    entry: Some("Updated content".to_string()),
    selected_tags: vec![],
    user_id: user.id.clone(),
  });

  assert!(edited.is_err());
}

#[test]
fn edit_entry_content_too_long() {
  let user = create_user();
  let entry = log::create_entry(log::CreateEntry {
    date: "2025-10-17".to_string(),
    mood: 3,
    entry: Some("Original content".to_string()),
    selected_tags: vec![],
    user_id: user.id.clone(),
  })
  .unwrap();

  let long_content = "a".repeat(1001);
  let edited = log::edit_entry(log::EditEntry {
    id: entry.id.clone(),
    date: "2025-10-18".to_string(),
    mood: 4,
    entry: Some(long_content),
    selected_tags: vec![],
    user_id: user.id.clone(),
  });

  assert!(edited.is_err());
}

#[test]
fn delete_tag_in_use_by_entry() {
  let user = create_user();
  let category = log::create_category(log::CreateCategory {
    name: "Test Category".to_string(),
    user_id: user.id.clone(),
  })
  .unwrap();
  let tag = log::create_tag(log::CreateTag {
    name: "In Use".to_string(),
    color: "blue".to_string(),
    category_id: category.id.clone(),
    user_id: user.id.clone(),
  })
  .unwrap();
  let entry = log::create_entry(log::CreateEntry {
    date: "2025-10-17".to_string(),
    mood: 5,
    entry: Some("Test entry".to_string()),
    selected_tags: vec![tag.id.clone()],
    user_id: user.id.clone(),
  })
  .unwrap();

  let deleted = log::delete_tag(&tag.id, &user.id);

  let get_entry_again = log::get_entry_with_tags(&entry.id, &user.id);

  assert!(deleted.is_ok());
  assert!(deleted.unwrap());
  assert!(get_entry_again.is_ok());
  let entry_with_tags = get_entry_again.unwrap();
  assert!(entry_with_tags.selected_tags.is_empty());
}

#[test]
fn delete_category_with_tags() {
  let user = create_user();
  let category = log::create_category(log::CreateCategory {
    name: "To Delete".to_string(),
    user_id: user.id.clone(),
  })
  .unwrap();
  let tag = log::create_tag(log::CreateTag {
    name: "Tag".to_string(),
    color: "blue".to_string(),
    category_id: category.id.clone(),
    user_id: user.id.clone(),
  })
  .unwrap();

  let deleted = log::delete_category(&category.id, &user.id);

  let found_tag = log::get_tag(&tag.id, &user.id);

  assert!(deleted.is_ok());
  assert!(deleted.unwrap());
  assert!(found_tag.is_err());
}

#[test]
fn create_entry_date_validation() {
  let user = create_user();

  let string_date = log::create_entry(log::CreateEntry {
    date: "invalid-date".to_string(),
    mood: 3,
    entry: Some("Test content".to_string()),
    selected_tags: vec![],
    user_id: user.id.clone(),
  });
  let empty_date = log::create_entry(log::CreateEntry {
    date: "".to_string(),
    mood: 3,
    entry: Some("Test content".to_string()),
    selected_tags: vec![],
    user_id: user.id.clone(),
  });
  let invalid_format_date = log::create_entry(log::CreateEntry {
    date: "2025/10/17".to_string(),
    mood: 3,
    entry: Some("Test content".to_string()),
    selected_tags: vec![],
    user_id: user.id.clone(),
  });
  let american_format_date = log::create_entry(log::CreateEntry {
    date: "10-17-2025".to_string(),
    mood: 3,
    entry: Some("Test content".to_string()),
    selected_tags: vec![],
    user_id: user.id.clone(),
  });

  assert!(string_date.is_err());
  assert!(empty_date.is_err());
  assert!(invalid_format_date.is_err());
  assert!(american_format_date.is_err());
}

#[test]
fn tag_color_default() {
  let user = create_user();
  let category = log::create_category(log::CreateCategory {
    name: "Test Category".to_string(),
    user_id: user.id.clone(),
  })
  .unwrap();

  let invalid_color_tag = log::create_tag(log::CreateTag {
    name: "Invalid Color".to_string(),
    color: "invalid".to_string(),
    category_id: category.id.clone(),
    user_id: user.id.clone(),
  });

  assert!(invalid_color_tag.is_ok());
  let tag = invalid_color_tag.unwrap();
  assert_eq!(tag.color, "base");
}

#[test]
fn get_entries_in_range() {
  let user = create_user();

  let entry1 = log::create_entry(log::CreateEntry {
    date: "2025-10-15".to_string(),
    mood: 4,
    entry: Some("Entry 1".to_string()),
    selected_tags: vec![],
    user_id: user.id.clone(),
  })
  .unwrap();

  let entry2 = log::create_entry(log::CreateEntry {
    date: "2025-10-16".to_string(),
    mood: 5,
    entry: Some("Entry 2".to_string()),
    selected_tags: vec![],
    user_id: user.id.clone(),
  })
  .unwrap();

  let entry3 = log::create_entry(log::CreateEntry {
    date: "2025-10-17".to_string(),
    mood: 3,
    entry: Some("Entry 3".to_string()),
    selected_tags: vec![],
    user_id: user.id.clone(),
  })
  .unwrap();

  let entry4 = log::create_entry(log::CreateEntry {
    date: "2025-10-14".to_string(),
    mood: 2,
    entry: Some("Entry 4".to_string()),
    selected_tags: vec![],
    user_id: user.id.clone(),
  })
  .unwrap();

  let options = log::GetEntriesOptions {
    from_date: Some("2025-10-15".to_string()),
    to_date: Some("2025-10-16".to_string()),
    ..Default::default()
  };
  let entries = log::get_entries(&user.id, Some(options));

  assert!(entries.is_ok());
  let entries = entries.unwrap();
  assert_eq!(entries.data.len(), 2);
  let entry_ids: Vec<String> = entries.data.into_iter().map(|e| e.id).collect();
  assert!(entry_ids.contains(&entry1.id));
  assert!(entry_ids.contains(&entry2.id));
  assert!(!entry_ids.contains(&entry3.id));
  assert!(!entry_ids.contains(&entry4.id));
}

#[test]
fn get_all_entries_no_options() {
  let user = create_user();

  let entry1 = log::create_entry(log::CreateEntry {
    date: "2025-10-15".to_string(),
    mood: 4,
    entry: Some("Entry 1".to_string()),
    selected_tags: vec![],
    user_id: user.id.clone(),
  })
  .unwrap();

  let entry2 = log::create_entry(log::CreateEntry {
    date: "2025-10-16".to_string(),
    mood: 5,
    entry: Some("Entry 2".to_string()),
    selected_tags: vec![],
    user_id: user.id.clone(),
  })
  .unwrap();

  let entries = log::get_entries(&user.id, None);

  assert!(entries.is_ok());
  let entries = entries.unwrap();
  assert!(entries.data.len() >= 2);
  let entry_ids: Vec<String> = entries.data.into_iter().map(|e| e.id).collect();
  assert!(entry_ids.contains(&entry1.id));
  assert!(entry_ids.contains(&entry2.id));
}

#[test]
fn get_entries_in_mood_range() {
  let user = create_user();

  let entry1 = log::create_entry(log::CreateEntry {
    date: "2025-10-15".to_string(),
    mood: 2,
    entry: Some("Entry 1".to_string()),
    selected_tags: vec![],
    user_id: user.id.clone(),
  })
  .unwrap();

  let entry2 = log::create_entry(log::CreateEntry {
    date: "2025-10-16".to_string(),
    mood: 4,
    entry: Some("Entry 2".to_string()),
    selected_tags: vec![],
    user_id: user.id.clone(),
  })
  .unwrap();

  let entry3 = log::create_entry(log::CreateEntry {
    date: "2025-10-17".to_string(),
    mood: 5,
    entry: Some("Entry 3".to_string()),
    selected_tags: vec![],
    user_id: user.id.clone(),
  })
  .unwrap();

  let options = log::GetEntriesOptions {
    from_mood: Some(3),
    to_mood: Some(5),
    ..Default::default()
  };
  let entries = log::get_entries(&user.id, Some(options));

  assert!(entries.is_ok());
  let entries = entries.unwrap();
  assert_eq!(entries.data.len(), 2);
  let entry_ids: Vec<String> = entries.data.into_iter().map(|e| e.id).collect();
  assert!(!entry_ids.contains(&entry1.id));
  assert!(entry_ids.contains(&entry2.id));
  assert!(entry_ids.contains(&entry3.id));
}

#[test]
fn get_entries_with_tags() {
  let user = create_user();
  let category = log::create_category(log::CreateCategory {
    name: "Test Category".to_string(),
    user_id: user.id.clone(),
  })
  .unwrap();
  let tag = log::create_tag(log::CreateTag {
    name: "Test Tag".to_string(),
    color: "blue".to_string(),
    category_id: category.id.clone(),
    user_id: user.id.clone(),
  })
  .unwrap();

  let entry = log::create_entry(log::CreateEntry {
    date: "2025-10-17".to_string(),
    mood: 5,
    entry: Some("Test entry".to_string()),
    selected_tags: vec![tag.id.clone()],
    user_id: user.id.clone(),
  })
  .unwrap();

  let _entry_no_tag = log::create_entry(log::CreateEntry {
    date: "2025-10-18".to_string(),
    mood: 4,
    entry: Some("Another entry".to_string()),
    selected_tags: vec![],
    user_id: user.id.clone(),
  })
  .unwrap();

  let options = log::GetEntriesOptions {
    tags: Some(vec![tag.id.clone()]),
    ..Default::default()
  };
  let entries = log::get_entries(&user.id, Some(options));

  assert!(entries.is_ok());
  let entries = entries.unwrap();
  assert_eq!(entries.data.len(), 1);
  assert_eq!(entries.data[0].id, entry.id);
}

#[test]
fn delete_category_with_tags_where_tags_are_also_in_use() {
  let user = create_user();
  let category = log::create_category(log::CreateCategory {
    name: "To Delete".to_string(),
    user_id: user.id.clone(),
  })
  .unwrap();
  let tag = log::create_tag(log::CreateTag {
    name: "Tag".to_string(),
    color: "blue".to_string(),
    category_id: category.id.clone(),
    user_id: user.id.clone(),
  })
  .unwrap();
  let entry = log::create_entry(log::CreateEntry {
    date: "2025-10-17".to_string(),
    mood: 5,
    entry: Some("Test entry".to_string()),
    selected_tags: vec![tag.id.clone()],
    user_id: user.id.clone(),
  })
  .unwrap();

  let deleted = log::delete_category(&category.id, &user.id);

  let found_tag = log::get_tag(&tag.id, &user.id);
  let found_entry = log::get_entry_with_tags(&entry.id, &user.id);

  assert!(deleted.is_ok());
  assert!(deleted.unwrap());
  assert!(found_tag.is_err());
  assert!(found_entry.is_ok());
  let entry_with_tags = found_entry.unwrap();
  assert!(entry_with_tags.selected_tags.is_empty());
}

#[test]
fn get_entries_limit_and_offset() {
  let user = create_user();

  for i in 1..=10 {
    let _ = log::create_entry(log::CreateEntry {
      date: format!("2025-10-{i:02}"),
      mood: (i % 5) + 1,
      entry: Some(format!("Entry {i}")),
      selected_tags: vec![],
      user_id: user.id.clone(),
    });
  }

  let options = log::GetEntriesOptions {
    limit: Some(3),
    offset: Some(4),
    ..Default::default()
  };
  let entries = log::get_entries(&user.id, Some(options));

  assert!(entries.is_ok());
  let entries = entries.unwrap();
  assert_eq!(entries.data.len(), 3);
  assert_eq!(entries.pagination.total_count, 10);
  assert_eq!(entries.pagination.limit, 3);
  assert_eq!(entries.pagination.offset, 4);
  assert_eq!(entries.data[0].entry.as_ref().unwrap(), "Entry 6");
  assert_eq!(entries.data[1].entry.as_ref().unwrap(), "Entry 5");
  assert_eq!(entries.data[2].entry.as_ref().unwrap(), "Entry 4");
}

#[test]
fn get_entries_limit_0_selects_more_than_31() {
  let user = create_user();

  for i in 1..=35 {
    let year = 1998 + i;
    let _ = log::create_entry(log::CreateEntry {
      date: format!("{year}-10-01"),
      mood: (i % 5) + 1,
      entry: Some(format!("Entry {i}")),
      selected_tags: vec![],
      user_id: user.id.clone(),
    });
  }

  let options = log::GetEntriesOptions {
    limit: Some(0),
    ..Default::default()
  };
  let entries = log::get_entries(&user.id, Some(options));

  assert!(entries.is_ok());
  let entries = entries.unwrap();
  assert_eq!(entries.data.len(), 35);
}

#[test]
fn delete_user_deletes_all_data() {
  let user = create_user();
  let category = log::create_category(log::CreateCategory {
    name: "Test Category".to_string(),
    user_id: user.id.clone(),
  })
  .unwrap();
  let tag = log::create_tag(log::CreateTag {
    name: "Test Tag".to_string(),
    color: "blue".to_string(),
    category_id: category.id.clone(),
    user_id: user.id.clone(),
  })
  .unwrap();
  let entry = log::create_entry(log::CreateEntry {
    date: "2025-10-17".to_string(),
    mood: 5,
    entry: Some("Test entry".to_string()),
    selected_tags: vec![tag.id.clone()],
    user_id: user.id.clone(),
  })
  .unwrap();

  let deleted = user::delete_user(&user.id);

  let found_category = log::get_category(&category.id, &user.id);
  let found_tag = log::get_tag(&tag.id, &user.id);
  let found_entry = log::get_entry_with_tags(&entry.id, &user.id);

  assert!(deleted.is_ok());
  assert!(deleted.unwrap());
  assert!(found_category.is_err());
  assert!(found_tag.is_err());
  assert!(found_entry.is_err());
}

#[test]
fn delete_tag_in_use() {
  let user = create_user();
  let category = log::create_category(log::CreateCategory {
    name: "Test Category".to_string(),
    user_id: user.id.clone(),
  })
  .unwrap();
  let tag = log::create_tag(log::CreateTag {
    name: "In Use".to_string(),
    color: "blue".to_string(),
    category_id: category.id.clone(),
    user_id: user.id.clone(),
  })
  .unwrap();
  let entry = log::create_entry(log::CreateEntry {
    date: "2025-10-17".to_string(),
    mood: 5,
    entry: Some("Test entry".to_string()),
    selected_tags: vec![tag.id.clone()],
    user_id: user.id.clone(),
  })
  .unwrap();

  let deleted = log::delete_tag(&tag.id, &user.id);

  assert!(deleted.is_ok());
  assert!(deleted.unwrap());

  let found_entry = log::get_entry_with_tags(&entry.id, &user.id);
  assert!(found_entry.is_ok());
  let entry_with_tags = found_entry.unwrap();
  assert!(entry_with_tags.selected_tags.is_empty());
}

#[test]
fn delete_entry_with_tags() {
  let user = create_user();
  let category = log::create_category(log::CreateCategory {
    name: "Test Category".to_string(),
    user_id: user.id.clone(),
  })
  .unwrap();
  let tag = log::create_tag(log::CreateTag {
    name: "Test Tag".to_string(),
    color: "blue".to_string(),
    category_id: category.id.clone(),
    user_id: user.id.clone(),
  })
  .unwrap();

  let entry = log::create_entry(log::CreateEntry {
    date: "2025-10-17".to_string(),
    mood: 5,
    entry: Some("Test entry".to_string()),
    selected_tags: vec![tag.id.clone()],
    user_id: user.id.clone(),
  })
  .unwrap();

  let deleted = log::delete_entry(&entry.id, &user.id);

  let found_entry = log::get_entry_with_tags(&entry.id, &user.id);
  let found_tag = log::get_tag(&tag.id, &user.id);

  assert!(deleted.is_ok());
  assert!(deleted.unwrap());
  assert!(found_entry.is_err());
  assert!(found_tag.is_ok());
}

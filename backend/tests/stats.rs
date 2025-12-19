use diarycomputer::services::{category, entry, stats, user};
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
fn mood_stats() {
  let user = create_user();

  // create several entries
  // 2 with mood 5
  // 7 with mood 4
  // 12 with mood 3
  // 3 with mood 2
  // 1 with mood 1
  // average mood = (2*5 + 7*4 + 12*3 + 3*2 + 1*1) / 25 = 3.24
  let mut year = 2025;
  let mut create_entry_with_mood = |mood: i32, count: i32| {
    for _ in 0..count {
      entry::create_entry(entry::CreateEntry {
        date: format!("{}-10-17", year),
        mood,
        entry: Some("Test entry content".to_string()),
        selected_tags: vec![],
        user_id: user.id.clone(),
      })
      .unwrap();
      year += 1;
    }
  };

  create_entry_with_mood(5, 2);
  create_entry_with_mood(4, 7);
  create_entry_with_mood(3, 12);
  create_entry_with_mood(2, 3);
  create_entry_with_mood(1, 1);

  let stats = stats::mood_stats(&user.id).unwrap();

  assert_eq!(stats.entry_count, 25);
  assert_eq!(stats.average_mood, 3.24);

  assert_eq!(stats.mood_entry_count.mood_1, 1);
  assert_eq!(stats.mood_entry_count.mood_2, 3);
  assert_eq!(stats.mood_entry_count.mood_3, 12);
  assert_eq!(stats.mood_entry_count.mood_4, 7);
  assert_eq!(stats.mood_entry_count.mood_5, 2);
}

#[test]
fn mood_stats_no_entries() {
  let user = create_user();
  let stats = stats::mood_stats(&user.id).unwrap();
  assert_eq!(stats.entry_count, 0);
  assert_eq!(stats.average_mood, 0.0);
}

#[test]
fn mood_stats_one_entry() {
  let user = create_user();

  entry::create_entry(entry::CreateEntry {
    date: "2025-10-17".to_string(),
    mood: 4,
    entry: Some("Test entry content".to_string()),
    selected_tags: vec![],
    user_id: user.id.clone(),
  })
  .unwrap();

  let stats = stats::mood_stats(&user.id).unwrap();

  assert_eq!(stats.entry_count, 1);
  assert_eq!(stats.average_mood, 4.0);
}

#[test]
fn tag_stats() {
  let user = create_user();
  let category = category::create_category(category::CreateCategory {
    name: "Test Category".to_string(),
    user_id: user.id.clone(),
  })
  .unwrap();

  // create tags
  let tag1 = diarycomputer::services::tag::create_tag(diarycomputer::services::tag::CreateTag {
    name: "Tag 1".to_string(),
    color: "red".to_string(),
    category_id: category.id.clone(),
    user_id: user.id.clone(),
  })
  .unwrap();

  let tag2 = diarycomputer::services::tag::create_tag(diarycomputer::services::tag::CreateTag {
    name: "Tag 2".to_string(),
    color: "base".to_string(),
    category_id: category.id.clone(),
    user_id: user.id.clone(),
  })
  .unwrap();

  let mut year = 2025;
  let mut create_entry_with_mood = |mood: i32, count: i32, tags: Vec<String>| {
    for _ in 0..count {
      entry::create_entry(entry::CreateEntry {
        date: format!("{}-10-17", year),
        mood,
        entry: Some("Test entry content".to_string()),
        selected_tags: tags.clone(),
        user_id: user.id.clone(),
      })
      .unwrap();
      year += 1;
    }
  };

  // create entries with tags
  // | mood | count | tags        |
  // |------|-------|-------------|
  // | 5    | 2     | [tag1]      |
  // | 5    | 3     | [tag2]      |
  // | 5    | 1     | [tag1, tag2]|
  // | 4    | 4     | [tag1]      |
  // | 2    | 2     | [tag2]      |
  // | 1    | 2     | [tag1, tag2]|

  // expected stats:
  // tag1: entry_count = 9, average_mood = (5*2 + 5*1 + 4*4 + 1*2) / 9 = 3.67
  // tag2: entry_count = 8, average_mood = (5*3 + 5*1 + 2*2 + 1*2) / 8 = 3.25

  create_entry_with_mood(5, 2, vec![tag1.id.clone()]);
  create_entry_with_mood(5, 3, vec![tag2.id.clone()]);
  create_entry_with_mood(5, 1, vec![tag1.id.clone(), tag2.id.clone()]);
  create_entry_with_mood(4, 4, vec![tag1.id.clone()]);
  create_entry_with_mood(2, 2, vec![tag2.id.clone()]);
  create_entry_with_mood(1, 2, vec![tag1.id.clone(), tag2.id.clone()]);

  let stats = stats::tag_stats(&user.id).unwrap();

  let tag1_stats = stats.iter().find(|s| s.tag_id == tag1.id).unwrap();
  assert_eq!(tag1_stats.entry_count, 9);
  assert_eq!(tag1_stats.average_mood, 3.67);

  assert_eq!(tag1_stats.mood_entry_count.mood_1, 2);
  assert_eq!(tag1_stats.mood_entry_count.mood_2, 0);
  assert_eq!(tag1_stats.mood_entry_count.mood_3, 0);
  assert_eq!(tag1_stats.mood_entry_count.mood_4, 4);
  assert_eq!(tag1_stats.mood_entry_count.mood_5, 3);

  let tag2_stats = stats.iter().find(|s| s.tag_id == tag2.id).unwrap();
  assert_eq!(tag2_stats.entry_count, 8);
  assert_eq!(tag2_stats.average_mood, 3.25);

  assert_eq!(tag2_stats.mood_entry_count.mood_1, 2);
  assert_eq!(tag2_stats.mood_entry_count.mood_2, 2);
  assert_eq!(tag2_stats.mood_entry_count.mood_3, 0);
  assert_eq!(tag2_stats.mood_entry_count.mood_4, 0);
  assert_eq!(tag2_stats.mood_entry_count.mood_5, 4);
}

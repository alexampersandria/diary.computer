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

  let mut date = chrono::NaiveDate::from_ymd_opt(2025, 12, 19).unwrap();
  let mut create_entry_with_mood = |mood: i32, count: i32| {
    for _ in 0..count {
      entry::create_entry(entry::CreateEntry {
        date: date.to_string(),
        mood,
        entry: Some("Test entry content".to_string()),
        selected_tags: vec![],
        user_id: user.id.clone(),
      })
      .unwrap();
      date = date.succ_opt().unwrap();
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
  assert_eq!(stats.median_mood, 3);

  let stats_with_count = stats::mood_stats_with_count(&user.id).unwrap();

  assert_eq!(stats_with_count.mood_entry_count.mood_1, 1);
  assert_eq!(stats_with_count.mood_entry_count.mood_2, 3);
  assert_eq!(stats_with_count.mood_entry_count.mood_3, 12);
  assert_eq!(stats_with_count.mood_entry_count.mood_4, 7);
  assert_eq!(stats_with_count.mood_entry_count.mood_5, 2);
}

#[test]
fn mood_stats_no_entries() {
  let user = create_user();
  let stats = stats::mood_stats(&user.id).unwrap();
  assert_eq!(stats.entry_count, 0);
  assert_eq!(stats.average_mood, 0.0);
  let tag_stats = stats::tag_stats(&user.id).unwrap();
  assert!(tag_stats.is_empty());
  let weekday_stats = stats::weekday_stats(&user.id).unwrap();
  assert_eq!(weekday_stats.monday.entry_count, 0);
  assert_eq!(weekday_stats.tuesday.entry_count, 0);
  assert_eq!(weekday_stats.wednesday.entry_count, 0);
  assert_eq!(weekday_stats.thursday.entry_count, 0);
  assert_eq!(weekday_stats.friday.entry_count, 0);
  assert_eq!(weekday_stats.saturday.entry_count, 0);
  assert_eq!(weekday_stats.sunday.entry_count, 0);
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
  assert_eq!(stats.median_mood, 4);
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
  let stats_with_count = stats::tag_stats_with_count(&user.id).unwrap();

  let tag1_stats = stats.iter().find(|s| s.tag_id == tag1.id).unwrap();
  assert_eq!(tag1_stats.entry_count, 9);
  assert_eq!(tag1_stats.average_mood, 3.67);

  let tag1_stats_with_count = stats_with_count
    .iter()
    .find(|s| s.tag_id == tag1.id)
    .unwrap();
  assert_eq!(tag1_stats_with_count.mood_entry_count.mood_1, 2);
  assert_eq!(tag1_stats_with_count.mood_entry_count.mood_2, 0);
  assert_eq!(tag1_stats_with_count.mood_entry_count.mood_3, 0);
  assert_eq!(tag1_stats_with_count.mood_entry_count.mood_4, 4);
  assert_eq!(tag1_stats_with_count.mood_entry_count.mood_5, 3);

  let tag2_stats = stats.iter().find(|s| s.tag_id == tag2.id).unwrap();
  assert_eq!(tag2_stats.entry_count, 8);
  assert_eq!(tag2_stats.average_mood, 3.25);

  let tag2_stats_with_count = stats_with_count
    .iter()
    .find(|s| s.tag_id == tag2.id)
    .unwrap();
  assert_eq!(tag2_stats_with_count.mood_entry_count.mood_1, 2);
  assert_eq!(tag2_stats_with_count.mood_entry_count.mood_2, 2);
  assert_eq!(tag2_stats_with_count.mood_entry_count.mood_3, 0);
  assert_eq!(tag2_stats_with_count.mood_entry_count.mood_4, 0);
  assert_eq!(tag2_stats_with_count.mood_entry_count.mood_5, 4);
}

#[test]
fn weekday_stats() {
  let user = create_user();

  // same as mood_stats because it was easier to copy paste
  // don't blame me
  // create several entries
  // 2 with mood 5
  // 7 with mood 4
  // 12 with mood 3
  // 3 with mood 2
  // 1 with mood 1
  // average mood = (2*5 + 7*4 + 12*3 + 3*2 + 1*1) / 25 = 3.24

  // starting date
  // friday, December 19, 2025
  let mut date = chrono::NaiveDate::from_ymd_opt(2025, 12, 19).unwrap();
  let mut create_entry_with_mood = |mood: i32, count: i32| {
    for _ in 0..count {
      entry::create_entry(entry::CreateEntry {
        date: date.to_string(),
        mood,
        entry: Some("Test entry content".to_string()),
        selected_tags: vec![],
        user_id: user.id.clone(),
      })
      .unwrap();
      date = date.succ_opt().unwrap();
    }
  };

  create_entry_with_mood(5, 2);
  create_entry_with_mood(4, 7);
  create_entry_with_mood(3, 12);
  create_entry_with_mood(2, 3);
  create_entry_with_mood(1, 1);

  // Check weekday stats
  // the two 5's were on friday and saturday
  // the seven 4's were on sunday, monday, tuesday, wednesday, thursday, friday, saturday
  // the twelve 3's were on sunday, monday, tuesday, wednesday, thursday, friday, saturday, sunday, monday, tuesday, wednesday, thursday
  // the three 2's were on friday, saturday, sunday
  // the one 1 was on monday
  // expected stats:
  // fridays: 4 entries, average mood = (5 + 4 + 3 + 2) / 4 = 3.5
  // saturdays: 4 entries, average mood = (5 + 4 + 3 + 2) / 4 = 3.5 // same as fridays
  // sundays: 4 entries, average mood = (4 + 3 + 3 + 2) / 4 = 3.0
  // mondays: 4 entries, average mood = (4 + 3 + 3 + 1 )/ 4 = 2.75
  // tuesdays: 3 entries, average mood = (4 + 3 + 3) / 3 = 3.33
  // wednesday: 3 entries, average mood = (4 + 3 + 3) / 3 = 3.33 // same as tuesdays
  // thursdays: 3 entries, average mood = (4 + 3 + 3) / 3 = 3.33 // same again :)
  let weekday_stats = stats::weekday_stats(&user.id).unwrap();
  assert_eq!(weekday_stats.friday.entry_count, 4);
  assert_eq!(weekday_stats.friday.average_mood, 3.5);
  assert_eq!(weekday_stats.saturday.entry_count, 4);
  assert_eq!(weekday_stats.saturday.average_mood, 3.5);
  assert_eq!(weekday_stats.sunday.entry_count, 4);
  assert_eq!(weekday_stats.sunday.average_mood, 3.0);
  assert_eq!(weekday_stats.monday.entry_count, 4);
  assert_eq!(weekday_stats.monday.average_mood, 2.75);
  assert_eq!(weekday_stats.tuesday.entry_count, 3);
  assert_eq!(weekday_stats.tuesday.average_mood, 3.33);
  assert_eq!(weekday_stats.wednesday.entry_count, 3);
  assert_eq!(weekday_stats.wednesday.average_mood, 3.33);
  assert_eq!(weekday_stats.wednesday.median_mood, 3);
  assert_eq!(weekday_stats.thursday.entry_count, 3);
  assert_eq!(weekday_stats.thursday.average_mood, 3.33);
}

#[test]
fn median_even_number_of_entries() {
  let user = create_user();

  // create entries with moods: 1, 2, 3, 4
  for (i, mood) in [1, 2, 3, 4].iter().enumerate() {
    entry::create_entry(entry::CreateEntry {
      date: format!("2025-10-1{}", i + 1),
      mood: *mood,
      entry: Some("Test entry content".to_string()),
      selected_tags: vec![],
      user_id: user.id.clone(),
    })
    .unwrap();
  }

  let stats = stats::mood_stats(&user.id).unwrap();

  assert_eq!(stats.entry_count, 4);
  assert_eq!(stats.average_mood, 2.5);
  assert_eq!(stats.median_mood, 2); // median of [1,2,3,4] is (2+3)/2 = 2.5 -> floor to 2
}

#[test]
fn median_is_high() {
  let user = create_user();

  // create 10 entries with mood 5 and 1 entry with mood 1
  let mut date = chrono::NaiveDate::from_ymd_opt(2025, 11, 1).unwrap();
  for _ in 0..10 {
    entry::create_entry(entry::CreateEntry {
      date: date.format("%Y-%m-%d").to_string(),
      mood: 5,
      entry: Some("Test entry content".to_string()),
      selected_tags: vec![],
      user_id: user.id.clone(),
    })
    .unwrap();
    date = date.succ_opt().unwrap();
  }
  entry::create_entry(entry::CreateEntry {
    date: date.format("%Y-%m-%d").to_string(),
    mood: 1,
    entry: Some("Test entry content".to_string()),
    selected_tags: vec![],
    user_id: user.id.clone(),
  })
  .unwrap();

  let stats = stats::mood_stats(&user.id).unwrap();

  assert_eq!(stats.entry_count, 11);
  assert_eq!(stats.average_mood, 4.64);
  assert_eq!(stats.median_mood, 5);
}

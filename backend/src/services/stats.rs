use crate::{establish_connection, schema, services::user::get_user, util::error::APIError};
use bigdecimal::{BigDecimal, ToPrimitive};
use diesel::{
  dsl::{avg, count_star, sql},
  ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MoodCount {
  pub mood_1: i64,
  pub mood_2: i64,
  pub mood_3: i64,
  pub mood_4: i64,
  pub mood_5: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MoodStats {
  pub entry_count: i64,
  pub average_mood: f64,
  pub median_mood: i32,
  pub mood_entry_count: MoodCount,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TagStats {
  pub tag_id: String,
  pub entry_count: i64,
  pub average_mood: f64,
  pub median_mood: i32,
  pub mood_entry_count: MoodCount,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WeekdayStats {
  pub monday: MoodStats,
  pub tuesday: MoodStats,
  pub wednesday: MoodStats,
  pub thursday: MoodStats,
  pub friday: MoodStats,
  pub saturday: MoodStats,
  pub sunday: MoodStats,
}

/// Format average mood to two decimal places
pub fn format_average_mood(value: f64) -> f64 {
  (value * 100.0).round() / 100.0
}

/// Get mood statistics for a user
/// entry_count: total number of entries
/// average_mood: average mood value across all entries
pub fn mood_stats(user_id: &str) -> Result<MoodStats, APIError> {
  let user = get_user(&user_id);

  if user.is_err() {
    return Err(APIError::UserNotFound);
  }

  let mut conn = match establish_connection() {
    Ok(connection) => connection,
    Err(_) => return Err(APIError::DatabaseError),
  };

  let result = schema::entries::table
    .filter(schema::entries::user_id.eq(user_id))
    .select((
      count_star(),
      avg(schema::entries::mood),
      sql::<diesel::sql_types::Nullable<diesel::sql_types::Integer>>(
        "PERCENTILE_DISC(0.5) WITHIN GROUP (ORDER BY mood)",
      ),
    ))
    .get_result::<(i64, Option<BigDecimal>, Option<i32>)>(&mut conn);

  let (entry_count, average_mood, median_mood) = match result {
    Ok(data) => data,
    Err(_) => return Err(APIError::DatabaseError),
  };

  // Get mood counts for each mood level
  let mood_1 = schema::entries::table
    .filter(schema::entries::user_id.eq(user_id))
    .filter(schema::entries::mood.eq(1))
    .count()
    .get_result::<i64>(&mut conn)
    .unwrap_or(0);

  let mood_2 = schema::entries::table
    .filter(schema::entries::user_id.eq(user_id))
    .filter(schema::entries::mood.eq(2))
    .count()
    .get_result::<i64>(&mut conn)
    .unwrap_or(0);

  let mood_3 = schema::entries::table
    .filter(schema::entries::user_id.eq(user_id))
    .filter(schema::entries::mood.eq(3))
    .count()
    .get_result::<i64>(&mut conn)
    .unwrap_or(0);

  let mood_4 = schema::entries::table
    .filter(schema::entries::user_id.eq(user_id))
    .filter(schema::entries::mood.eq(4))
    .count()
    .get_result::<i64>(&mut conn)
    .unwrap_or(0);

  let mood_5 = schema::entries::table
    .filter(schema::entries::user_id.eq(user_id))
    .filter(schema::entries::mood.eq(5))
    .count()
    .get_result::<i64>(&mut conn)
    .unwrap_or(0);

  Ok(MoodStats {
    entry_count,
    average_mood: format_average_mood(average_mood.and_then(|v| v.to_f64()).unwrap_or(0.0)),
    median_mood: median_mood.unwrap_or(0),
    mood_entry_count: MoodCount {
      mood_1,
      mood_2,
      mood_3,
      mood_4,
      mood_5,
    },
  })
}

/// Get tag statistics for a user
/// Returns a vector of TagStats, each containing:
/// tag_id: ID of the tag
/// entry_count: number of entries associated with the tag
/// average_mood: average mood of entries associated with the tag
pub fn tag_stats(user_id: &str) -> Result<Vec<TagStats>, APIError> {
  let user = get_user(&user_id);

  if user.is_err() {
    return Err(APIError::UserNotFound);
  }

  let mut conn = match establish_connection() {
    Ok(connection) => connection,
    Err(_) => return Err(APIError::DatabaseError),
  };

  let results = schema::entry_tags::table
    .inner_join(schema::entries::table.on(schema::entry_tags::entry_id.eq(schema::entries::id)))
    .filter(schema::entries::user_id.eq(user_id))
    .group_by(schema::entry_tags::tag_id)
    .select((
      schema::entry_tags::tag_id,
      count_star(),
      avg(schema::entries::mood),
      sql::<diesel::sql_types::Nullable<diesel::sql_types::Integer>>(
        "PERCENTILE_DISC(0.5) WITHIN GROUP (ORDER BY entries.mood)",
      ),
    ))
    .load::<(String, i64, Option<BigDecimal>, Option<i32>)>(&mut conn);

  match results {
    Ok(rows) => Ok(
      rows
        .into_iter()
        .map(|(tag_id, entry_count, average_mood, median_mood)| {
          // Get mood counts for this tag
          let mood_1 = schema::entry_tags::table
            .inner_join(
              schema::entries::table.on(schema::entry_tags::entry_id.eq(schema::entries::id)),
            )
            .filter(schema::entries::user_id.eq(user_id))
            .filter(schema::entry_tags::tag_id.eq(&tag_id))
            .filter(schema::entries::mood.eq(1))
            .count()
            .get_result::<i64>(&mut conn)
            .unwrap_or(0);

          let mood_2 = schema::entry_tags::table
            .inner_join(
              schema::entries::table.on(schema::entry_tags::entry_id.eq(schema::entries::id)),
            )
            .filter(schema::entries::user_id.eq(user_id))
            .filter(schema::entry_tags::tag_id.eq(&tag_id))
            .filter(schema::entries::mood.eq(2))
            .count()
            .get_result::<i64>(&mut conn)
            .unwrap_or(0);

          let mood_3 = schema::entry_tags::table
            .inner_join(
              schema::entries::table.on(schema::entry_tags::entry_id.eq(schema::entries::id)),
            )
            .filter(schema::entries::user_id.eq(user_id))
            .filter(schema::entry_tags::tag_id.eq(&tag_id))
            .filter(schema::entries::mood.eq(3))
            .count()
            .get_result::<i64>(&mut conn)
            .unwrap_or(0);

          let mood_4 = schema::entry_tags::table
            .inner_join(
              schema::entries::table.on(schema::entry_tags::entry_id.eq(schema::entries::id)),
            )
            .filter(schema::entries::user_id.eq(user_id))
            .filter(schema::entry_tags::tag_id.eq(&tag_id))
            .filter(schema::entries::mood.eq(4))
            .count()
            .get_result::<i64>(&mut conn)
            .unwrap_or(0);

          let mood_5 = schema::entry_tags::table
            .inner_join(
              schema::entries::table.on(schema::entry_tags::entry_id.eq(schema::entries::id)),
            )
            .filter(schema::entries::user_id.eq(user_id))
            .filter(schema::entry_tags::tag_id.eq(&tag_id))
            .filter(schema::entries::mood.eq(5))
            .count()
            .get_result::<i64>(&mut conn)
            .unwrap_or(0);

          TagStats {
            tag_id,
            entry_count,
            average_mood: format_average_mood(average_mood.and_then(|v| v.to_f64()).unwrap_or(0.0)),
            median_mood: median_mood.unwrap_or(0),
            mood_entry_count: MoodCount {
              mood_1,
              mood_2,
              mood_3,
              mood_4,
              mood_5,
            },
          }
        })
        .collect(),
    ),
    Err(_) => Err(APIError::DatabaseError),
  }
}

/// Helper function to get mood stats for a specific weekday
/// day_name: day of week as string (e.g., 'Monday', 'Tuesday', etc.)
fn mood_stats_for_weekday(
  user_id: &str,
  day_name: &str,
  conn: &mut diesel::PgConnection,
) -> MoodStats {
  let result = schema::entries::table
    .filter(schema::entries::user_id.eq(user_id))
    .filter(
      sql::<diesel::sql_types::Bool>("TRIM(TO_CHAR(date, 'Day')) = ")
        .bind::<diesel::sql_types::Text, _>(day_name),
    )
    .select((
      count_star(),
      avg(schema::entries::mood),
      sql::<diesel::sql_types::Nullable<diesel::sql_types::Integer>>(
        "PERCENTILE_DISC(0.5) WITHIN GROUP (ORDER BY mood)",
      ),
    ))
    .get_result::<(i64, Option<BigDecimal>, Option<i32>)>(conn);

  let (entry_count, average_mood, median_mood) = result.unwrap_or((0, None, None));

  // Get mood counts for each mood level
  let mood_1 = schema::entries::table
    .filter(schema::entries::user_id.eq(user_id))
    .filter(
      sql::<diesel::sql_types::Bool>("TRIM(TO_CHAR(date, 'Day')) = ")
        .bind::<diesel::sql_types::Text, _>(day_name),
    )
    .filter(schema::entries::mood.eq(1))
    .count()
    .get_result::<i64>(conn)
    .unwrap_or(0);

  let mood_2 = schema::entries::table
    .filter(schema::entries::user_id.eq(user_id))
    .filter(
      sql::<diesel::sql_types::Bool>("TRIM(TO_CHAR(date, 'Day')) = ")
        .bind::<diesel::sql_types::Text, _>(day_name),
    )
    .filter(schema::entries::mood.eq(2))
    .count()
    .get_result::<i64>(conn)
    .unwrap_or(0);

  let mood_3 = schema::entries::table
    .filter(schema::entries::user_id.eq(user_id))
    .filter(
      sql::<diesel::sql_types::Bool>("TRIM(TO_CHAR(date, 'Day')) = ")
        .bind::<diesel::sql_types::Text, _>(day_name),
    )
    .filter(schema::entries::mood.eq(3))
    .count()
    .get_result::<i64>(conn)
    .unwrap_or(0);

  let mood_4 = schema::entries::table
    .filter(schema::entries::user_id.eq(user_id))
    .filter(
      sql::<diesel::sql_types::Bool>("TRIM(TO_CHAR(date, 'Day')) = ")
        .bind::<diesel::sql_types::Text, _>(day_name),
    )
    .filter(schema::entries::mood.eq(4))
    .count()
    .get_result::<i64>(conn)
    .unwrap_or(0);

  let mood_5 = schema::entries::table
    .filter(schema::entries::user_id.eq(user_id))
    .filter(
      sql::<diesel::sql_types::Bool>("TRIM(TO_CHAR(date, 'Day')) = ")
        .bind::<diesel::sql_types::Text, _>(day_name),
    )
    .filter(schema::entries::mood.eq(5))
    .count()
    .get_result::<i64>(conn)
    .unwrap_or(0);

  MoodStats {
    entry_count,
    average_mood: format_average_mood(average_mood.and_then(|v| v.to_f64()).unwrap_or(0.0)),
    median_mood: median_mood.unwrap_or(0),
    mood_entry_count: MoodCount {
      mood_1,
      mood_2,
      mood_3,
      mood_4,
      mood_5,
    },
  }
}

/// Get weekday statistics for a user
pub fn weekday_stats(user_id: &str) -> Result<WeekdayStats, APIError> {
  let user = get_user(&user_id);

  if user.is_err() {
    return Err(APIError::UserNotFound);
  }

  let mut conn = match establish_connection() {
    Ok(connection) => connection,
    Err(_) => return Err(APIError::DatabaseError),
  };

  Ok(WeekdayStats {
    monday: mood_stats_for_weekday(user_id, "Monday", &mut conn),
    tuesday: mood_stats_for_weekday(user_id, "Tuesday", &mut conn),
    wednesday: mood_stats_for_weekday(user_id, "Wednesday", &mut conn),
    thursday: mood_stats_for_weekday(user_id, "Thursday", &mut conn),
    friday: mood_stats_for_weekday(user_id, "Friday", &mut conn),
    saturday: mood_stats_for_weekday(user_id, "Saturday", &mut conn),
    sunday: mood_stats_for_weekday(user_id, "Sunday", &mut conn),
  })
}

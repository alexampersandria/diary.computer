use crate::{establish_connection, schema, services::user::get_user, util::error::APIError};
use bigdecimal::{BigDecimal, ToPrimitive};
use diesel::{
  dsl::{avg, count_star},
  ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MoodStats {
  pub entry_count: i64,
  pub average_mood: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TagStats {
  pub tag_id: String,
  pub entry_count: i64,
  pub average_mood: f64,
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

  match schema::entries::table
    .filter(schema::entries::user_id.eq(user_id))
    .select((count_star(), avg(schema::entries::mood)))
    .get_result::<(i64, Option<BigDecimal>)>(&mut conn)
  {
    Ok((entry_count, average_mood)) => Ok(MoodStats {
      entry_count,
      average_mood: format_average_mood(average_mood.and_then(|v| v.to_f64()).unwrap_or(0.0)),
    }),
    Err(_) => Err(APIError::DatabaseError),
  }
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
    ))
    .load::<(String, i64, Option<BigDecimal>)>(&mut conn);

  match results {
    Ok(rows) => Ok(
      rows
        .into_iter()
        .map(|(tag_id, entry_count, average_mood)| TagStats {
          tag_id,
          entry_count,
          average_mood: format_average_mood(average_mood.and_then(|v| v.to_f64()).unwrap_or(0.0)),
        })
        .collect(),
    ),
    Err(_) => Err(APIError::DatabaseError),
  }
}

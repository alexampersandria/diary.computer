use crate::{
  services::{authorize_request, log, GetEntriesOptions},
  util::{error::error_response, response},
};
use poem::{handler, http::StatusCode, web::Query, Request, Response};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EntryParams {
  pub from_date: Option<String>,
  pub to_date: Option<String>,
  pub tags: Option<String>,
  pub from_mood: Option<i32>,
  pub to_mood: Option<i32>,
  pub order: Option<String>,
  pub limit: Option<i64>,
  pub offset: Option<i64>,
}

#[handler]
pub async fn get_entries(Query(_options): Query<EntryParams>, request: &Request) -> Response {
  let session = match authorize_request(request).await {
    Ok(session) => session,
    Err(error) => return error_response(error),
  };

  let options = GetEntriesOptions {
    from_date: _options.from_date,
    to_date: _options.to_date,
    tags: _options
      .tags
      .map(|tags| tags.split(',').map(|s| s.to_string()).collect()),
    from_mood: _options.from_mood,
    to_mood: _options.to_mood,
    order: match _options.order.as_deref() {
      Some("date_asc") => Some(log::EntryOptionsOrder::DateAsc),
      Some("date_desc") => Some(log::EntryOptionsOrder::DateDesc),
      Some("mood_asc") => Some(log::EntryOptionsOrder::MoodAsc),
      Some("mood_desc") => Some(log::EntryOptionsOrder::MoodDesc),
      _ => None,
    },
    limit: _options.limit,
    offset: _options.offset,
  };

  let entries = log::get_entries(&session.user_id, Some(options));

  match entries {
    Ok(entries) => response(StatusCode::OK, &entries),
    Err(error) => error_response(error),
  }
}

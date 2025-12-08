use crate::{
  services::{
    auth::authorize_request,
    entry,
    entry::{CreateEntry, EditEntry},
  },
  util::{
    error::{error_response, APIError},
    response::response,
  },
};
use poem::{
  handler,
  http::StatusCode,
  web::{Json, Path},
  Request, Response,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct CreateEntryRequest {
  date: String,
  mood: i32,
  entry: Option<String>,
  selected_tags: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct EditEntryRequest {
  date: String,
  mood: i32,
  entry: Option<String>,
  selected_tags: Vec<String>,
}

#[handler]
pub async fn create_entry(Json(entry): Json<CreateEntryRequest>, request: &Request) -> Response {
  let session = match authorize_request(request).await {
    Ok(session) => session,
    Err(error) => return error_response(error),
  };

  let created_entry = entry::create_entry(CreateEntry {
    date: entry.date,
    mood: entry.mood,
    entry: entry.entry,
    selected_tags: entry.selected_tags,
    user_id: session.user_id,
  });

  match created_entry {
    Ok(created_entry) => response(StatusCode::CREATED, &created_entry),
    Err(error) => error_response(error),
  }
}

#[handler]
pub async fn edit_entry(
  Path(id): Path<String>,
  Json(entry): Json<EditEntryRequest>,
  request: &Request,
) -> Response {
  let session = match authorize_request(request).await {
    Ok(session) => session,
    Err(error) => return error_response(error),
  };

  let edited_entry = entry::edit_entry(EditEntry {
    id,
    date: entry.date,
    mood: entry.mood,
    entry: entry.entry,
    selected_tags: entry.selected_tags,
    user_id: session.user_id,
  });

  match edited_entry {
    Ok(edited_entry) => response(StatusCode::OK, &edited_entry),
    Err(error) => error_response(error),
  }
}

#[handler]
pub async fn delete_entry(Path(id): Path<String>, request: &Request) -> Response {
  let session = match authorize_request(request).await {
    Ok(session) => session,
    Err(error) => return error_response(error),
  };

  let deleted_entry = match entry::delete_entry(&id, &session.user_id) {
    Ok(deleted) => deleted,
    Err(error) => return error_response(error),
  };

  match deleted_entry {
    true => response(StatusCode::NO_CONTENT, &()),
    false => error_response(APIError::EntryNotFound),
  }
}

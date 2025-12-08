use crate::{
  services::{
    auth::authorize_request,
    tag,
    tag::{CreateTag, EditTag},
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
struct CreateTagRequest {
  name: String,
  color: String,
  category_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct EditTagRequest {
  name: String,
  color: String,
  category_id: Option<String>,
}

#[handler]
pub async fn create_tag(Json(tag): Json<CreateTagRequest>, request: &Request) -> Response {
  let session = match authorize_request(request).await {
    Ok(session) => session,
    Err(error) => return error_response(error),
  };

  match tag::create_tag(CreateTag {
    name: tag.name,
    color: tag.color,
    category_id: tag.category_id,
    user_id: session.user_id,
  }) {
    Ok(created_tag) => response(StatusCode::CREATED, &created_tag),
    Err(error) => error_response(error),
  }
}

#[handler]
pub async fn edit_tag(
  Path(id): Path<String>,
  Json(tag): Json<EditTagRequest>,
  request: &Request,
) -> Response {
  let session = match authorize_request(request).await {
    Ok(session) => session,
    Err(error) => return error_response(error),
  };

  match tag::edit_tag(EditTag {
    id,
    name: tag.name,
    color: tag.color,
    category_id: tag.category_id,
    user_id: session.user_id,
  }) {
    Ok(edited_tag) => response(StatusCode::OK, &edited_tag),
    Err(error) => error_response(error),
  }
}

#[handler]
pub async fn delete_tag(Path(id): Path<String>, request: &Request) -> Response {
  let session = match authorize_request(request).await {
    Ok(session) => session,
    Err(error) => return error_response(error),
  };

  match tag::delete_tag(&id, &session.user_id) {
    Ok(deleted) => match deleted {
      true => response(StatusCode::NO_CONTENT, &()),
      false => error_response(APIError::TagNotFound),
    },
    Err(error) => error_response(error),
  }
}

use crate::{
  services::{authorize_request, log},
  util::{error::error_response, response, APIError},
};
use poem::{
  handler,
  http::StatusCode,
  web::{Json, Path},
  Request, Response,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct CreateCategoryRequest {
  name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct EditCategoryRequest {
  name: String,
}

#[handler]
pub async fn create_category(
  Json(category): Json<CreateCategoryRequest>,
  request: &Request,
) -> Response {
  let session = match authorize_request(request).await {
    Ok(session) => session,
    Err(error) => return error_response(error),
  };

  let created_category = log::create_category(log::CreateCategory {
    name: category.name,
    user_id: session.user_id,
  });

  match created_category {
    Ok(created_category) => response(StatusCode::CREATED, &created_category),
    Err(error) => error_response(error),
  }
}

#[handler]
pub async fn edit_category(
  Path(id): Path<String>,
  Json(category): Json<EditCategoryRequest>,
  request: &Request,
) -> Response {
  let session = match authorize_request(request).await {
    Ok(session) => session,
    Err(error) => return error_response(error),
  };

  let edited_category = log::edit_category(log::EditCategory {
    id,
    name: category.name,
    user_id: session.user_id,
  });

  match edited_category {
    Ok(edited_category) => response(StatusCode::OK, &edited_category),
    Err(error) => error_response(error),
  }
}

#[handler]
pub async fn delete_category(Path(id): Path<String>, request: &Request) -> Response {
  let session = match authorize_request(request).await {
    Ok(session) => session,
    Err(error) => return error_response(error),
  };

  let deleted_category = match log::delete_category(&id, &session.user_id) {
    Ok(deleted) => deleted,
    Err(error) => return error_response(error),
  };

  match deleted_category {
    true => response(StatusCode::NO_CONTENT, &()),
    false => error_response(APIError::CategoryNotFound),
  }
}

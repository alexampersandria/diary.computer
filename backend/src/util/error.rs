use crate::util::response::response;
use poem::{http::StatusCode, Response};
use serde::Serialize;

#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum APIError {
  Unauthorized,
  DatabaseError,
  InternalServerError,
  UserNotFound,
  InviteNotFound,
  SessionNotFound,
  CategoryNotFound,
  TagNotFound,
  EntryNotFound,
  EmailAlreadyInUse,
  InvalidPassword,
  InviteUsed,
  BadRequest,
  EntryAlreadyExistsForDate,
}

#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum ServiceError {
  MigrationError,
}

#[derive(Serialize)]
struct ErrorBody {
  code: APIError,
  message: String,
}

fn error_message(error: APIError) -> String {
  match error {
    APIError::Unauthorized => "Unauthorized",
    APIError::UserNotFound => "User not found",
    APIError::InviteNotFound => "Invite not found",
    APIError::SessionNotFound => "Session not found",
    APIError::CategoryNotFound => "Category not found",
    APIError::TagNotFound => "Tag not found",
    APIError::EntryNotFound => "Entry not found",
    APIError::EmailAlreadyInUse => "Email already in use",
    APIError::InvalidPassword => "Invalid password",
    APIError::InviteUsed => "Invite already used",
    APIError::BadRequest => "Bad request",
    APIError::EntryAlreadyExistsForDate => "An entry already exists for the given date",
    _ => "An error occurred",
  }
  .to_string()
}

fn status_code(error: APIError) -> StatusCode {
  match error {
    APIError::Unauthorized => StatusCode::UNAUTHORIZED,
    APIError::UserNotFound => StatusCode::NOT_FOUND,
    APIError::InviteNotFound => StatusCode::NOT_FOUND,
    APIError::SessionNotFound => StatusCode::NOT_FOUND,
    APIError::CategoryNotFound => StatusCode::NOT_FOUND,
    APIError::TagNotFound => StatusCode::NOT_FOUND,
    APIError::EntryNotFound => StatusCode::NOT_FOUND,
    APIError::EmailAlreadyInUse => StatusCode::CONFLICT,
    APIError::InvalidPassword => StatusCode::UNAUTHORIZED,
    APIError::InviteUsed => StatusCode::CONFLICT,
    APIError::BadRequest => StatusCode::BAD_REQUEST,
    APIError::EntryAlreadyExistsForDate => StatusCode::CONFLICT,
    _ => StatusCode::INTERNAL_SERVER_ERROR,
  }
}

fn error_body(error: APIError) -> ErrorBody {
  ErrorBody {
    code: error,
    message: error_message(error),
  }
}

pub fn error_response(error: APIError) -> Response {
  response(status_code(error), &error_body(error))
}

#[cfg(test)]
mod ci_unit {
  use super::*;

  #[test]
  fn test_error_response() {
    let error = APIError::Unauthorized;
    let response = error_response(error);
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
  }
}

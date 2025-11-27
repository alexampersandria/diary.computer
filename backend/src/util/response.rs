use crate::util::error::{error_response, APIError};
use poem::{http::StatusCode, Body, Response};

fn json_response(status_code: StatusCode, body: impl Into<Body>) -> Response {
  Response::builder()
    .status(status_code)
    .header("Content-Type", "application/json")
    .body(body)
}

pub fn response(status_code: StatusCode, body: &impl serde::Serialize) -> Response {
  let body = match serde_json::to_string(body) {
    Ok(body) => body,
    Err(_) => return error_response(APIError::InternalServerError),
  };
  json_response(status_code, body)
}

pub fn empty_response(status_code: StatusCode) -> Response {
  json_response(status_code, ())
}

#[cfg(test)]
mod ci_unit {
  use super::*;

  #[test]
  fn test_response() {
    let response = response(StatusCode::CREATED, &());
    assert_eq!(response.status(), StatusCode::CREATED);
    assert_eq!(
      response.headers().get("Content-Type").unwrap(),
      "application/json"
    );
  }

  #[test]
  fn test_empty_response() {
    let response = empty_response(StatusCode::CREATED);
    assert_eq!(response.status(), StatusCode::CREATED);
    assert_eq!(
      response.headers().get("Content-Type").unwrap(),
      "application/json"
    );
  }
}

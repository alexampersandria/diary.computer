use crate::{services::health::health_check, util::response::response};
use poem::{handler, http::StatusCode, Response};

#[handler]
pub async fn health() -> Response {
  response(StatusCode::OK, &health_check().await)
}

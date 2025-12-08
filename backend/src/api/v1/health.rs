use crate::{services::health_check, util::response};
use poem::{handler, http::StatusCode, Response};

#[handler]
pub async fn health() -> Response {
  response(StatusCode::OK, &health_check().await)
}

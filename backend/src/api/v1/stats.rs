use crate::{
  services::{auth::authorize_request, stats},
  util::{error::error_response, response::response},
};
use poem::{handler, http::StatusCode, Request, Response};

#[handler]
pub async fn mood_stats(request: &Request) -> Response {
  let session = match authorize_request(request).await {
    Ok(session) => session,
    Err(error) => return error_response(error),
  };

  match stats::mood_stats(&session.user_id) {
    Ok(mood_stats) => response(StatusCode::OK, &mood_stats),
    Err(error) => error_response(error),
  }
}

#[handler]
pub async fn tag_stats(request: &Request) -> Response {
  let session = match authorize_request(request).await {
    Ok(session) => session,
    Err(error) => return error_response(error),
  };

  match stats::tag_stats(&session.user_id) {
    Ok(tag_stats) => response(StatusCode::OK, &tag_stats),
    Err(error) => error_response(error),
  }
}

#[handler]
pub async fn weekda_stats(request: &Request) -> Response {
  let session = match authorize_request(request).await {
    Ok(session) => session,
    Err(error) => return error_response(error),
  };

  match stats::weekday_stats(&session.user_id) {
    Ok(weekday_stats) => response(StatusCode::OK, &weekday_stats),
    Err(error) => error_response(error),
  }
}

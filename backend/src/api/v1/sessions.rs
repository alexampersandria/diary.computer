use crate::{
  services::{auth, authorize_request},
  util::{error::error_response, response},
};
use poem::{handler, http::StatusCode, Request, Response};

#[handler]
pub async fn get_sessions(request: &Request) -> Response {
  let session = match authorize_request(request).await {
    Ok(session) => session,
    Err(error) => return error_response(error),
  };

  let sessions = auth::get_all_user_sessions(&session.user_id);

  match sessions {
    Ok(sessions) => response(StatusCode::OK, &sessions),
    Err(error) => error_response(error),
  }
}

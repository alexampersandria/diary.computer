use crate::{
  services::{auth, auth::authorize_request},
  util::{error::error_response, response::response},
};
use poem::{handler, http::StatusCode, web::Path, Request, Response};

#[handler]
pub async fn delete_session(Path(id): Path<String>, request: &Request) -> Response {
  match authorize_request(request).await {
    Ok(session) => session,
    Err(error) => return error_response(error),
  };

  match auth::delete_user_session(&id) {
    Ok(_) => response(StatusCode::NO_CONTENT, &""),
    Err(error) => error_response(error),
  }
}

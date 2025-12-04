use crate::{
  services::{auth, user, AuthConfig, UserCredentials},
  util::{
    error::{error_response, APIError},
    response,
  },
};
use poem::{handler, http::StatusCode, web::Json, Request, Response};

use validator::Validate;

use dotenvy::dotenv;
use std::env;

#[handler]
pub async fn authenticate_user(Json(user): Json<user::AuthUser>, request: &Request) -> Response {
  match user.validate() {
    Ok(_) => (),
    Err(_) => return error_response(APIError::BadRequest),
  }

  let metadata = auth::session_metadata(request).await;

  let session = auth::create_user_session(
    UserCredentials {
      email: String::from(&user.email),
      password: String::from(&user.password),
    },
    metadata,
  );

  match session {
    Ok(session) => response(StatusCode::CREATED, &session),
    Err(error) => error_response(error),
  }
}

#[handler]
pub async fn auth_config() -> Response {
  dotenv().ok();

  let auth_config: AuthConfig = AuthConfig {
    invite_required: env::var("INVITE_REQUIRED").unwrap_or("false".to_string()) == "true",
  };

  response(StatusCode::OK, &auth_config)
}

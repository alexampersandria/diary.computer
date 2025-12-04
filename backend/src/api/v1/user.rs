use crate::{
  services::{auth, authorize_request, invite, log, user, UserCredentials},
  util::{
    error::{error_response, APIError},
    response,
  },
};
use poem::{handler, http::StatusCode, web::Json, Request, Response};

use dotenvy::dotenv;
use std::env;

#[handler]
pub async fn create_user(Json(user): Json<user::CreateUser>, request: &Request) -> Response {
  dotenv().ok();

  if env::var("INVITE_REQUIRED").unwrap_or("false".to_string()) == "true" {
    match &user.invite {
      Some(invite) => match invite::use_invite(invite) {
        Ok(_) => (),
        Err(_) => return error_response(APIError::InviteNotFound),
      },
      None => return error_response(APIError::InviteNotFound),
    }
  }

  let password = user.password.clone();
  let created_user = match user::create_user(user) {
    Ok(user) => user,
    Err(error) => return error_response(error),
  };

  let session = auth::create_user_session(
    UserCredentials {
      email: created_user.email,
      password,
    },
    auth::session_metadata(request).await,
  );

  match session {
    Ok(session) => response(StatusCode::CREATED, &session),
    Err(error) => error_response(error),
  }
}

#[handler]
pub async fn get_current_user(request: &Request) -> Response {
  let session = match authorize_request(request).await {
    Ok(session) => session,
    Err(error) => return error_response(error),
  };

  let user = user::get_user(&session.user_id);

  match user {
    Ok(user) => response(StatusCode::OK, &user),
    Err(error) => error_response(error),
  }
}

#[handler]
pub async fn delete_user(request: &Request) -> Response {
  let session = match authorize_request(request).await {
    Ok(session) => session,
    Err(error) => return error_response(error),
  };

  let deleted = match user::delete_user(&session.user_id) {
    Ok(deleted) => deleted,
    Err(error) => return error_response(error),
  };

  match deleted {
    true => response(StatusCode::NO_CONTENT, &()),
    false => error_response(APIError::UserNotFound),
  }
}

#[handler]
pub async fn update_user(Json(user): Json<user::UpdateUser>, request: &Request) -> Response {
  let session = match authorize_request(request).await {
    Ok(session) => session,
    Err(error) => return error_response(error),
  };

  let updated_user = match user::update_user(&session.user_id, user) {
    Ok(user) => user,
    Err(error) => return error_response(error),
  };

  match updated_user {
    true => response(StatusCode::NO_CONTENT, &()),
    false => error_response(APIError::UserNotFound),
  }
}

#[handler]
pub async fn update_password(
  Json(password): Json<user::UpdatePassword>,
  request: &Request,
) -> Response {
  let session = match authorize_request(request).await {
    Ok(session) => session,
    Err(error) => return error_response(error),
  };

  let updated_password = match user::update_password(&session.user_id, password) {
    Ok(updated) => updated,
    Err(error) => return error_response(error),
  };

  match updated_password {
    true => response(StatusCode::NO_CONTENT, &()),
    false => error_response(APIError::UserNotFound),
  }
}

#[handler]
pub async fn get_user_categories_with_tags(request: &Request) -> Response {
  let session = match authorize_request(request).await {
    Ok(session) => session,
    Err(error) => return error_response(error),
  };

  let categories = match log::get_user_categories_with_tags(&session.user_id) {
    Ok(categories) => categories,
    Err(error) => return error_response(error),
  };

  response(StatusCode::OK, &categories)
}

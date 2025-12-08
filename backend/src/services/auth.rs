use crate::{
  establish_connection,
  schema::{self, sessions},
  services::user,
  util,
  util::error::APIError,
};
use diesel::{deserialize::Queryable, ExpressionMethods, Insertable, QueryDsl, RunQueryDsl};
use poem::{web::RealIp, FromRequest, Request};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Insertable, Queryable)]
pub struct Session {
  pub id: String,
  pub user_id: String,
  pub created_at: i64,
  pub accessed_at: i64,
  pub ip_address: String,
  pub user_agent: String,
}

pub struct SessionMetadata {
  pub ip_address: String,
  pub user_agent: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserCredentials {
  pub email: String,
  pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthConfig {
  pub invite_required: bool,
}

pub async fn session_metadata(request: &Request) -> SessionMetadata {
  let remote_addr = RealIp::from_request_without_body(request)
    .await
    .ok()
    .and_then(|real_ip| real_ip.0)
    .map(|addr| addr.to_string())
    .unwrap_or_else(|| request.remote_addr().to_string());

  SessionMetadata {
    ip_address: remote_addr,
    user_agent: request
      .header("user-agent")
      .unwrap_or("unknown")
      .to_string(),
  }
}

/// Extracts the Bearer token from the Authorization header
pub fn token_from_header(request: &Request) -> Option<String> {
  let token = request.header("Authorization");
  token.map(|token| token.replace("Bearer ", ""))
}

/// Authorizes a request by validating the session token from the Authorization header
/// and updates the session metadata
pub async fn authorize_request(request: &Request) -> Result<Session, APIError> {
  let token = match token_from_header(request) {
    Some(token) => token,
    None => return Err(APIError::Unauthorized),
  };

  match get_user_session_by_id(&token) {
    Ok(_) => (),
    Err(_) => return Err(APIError::Unauthorized),
  }

  match update_session(&token, request).await {
    Ok(session) => Ok(session),
    Err(error) => Err(error),
  }
}

pub fn create_user_session(
  user_credentials: UserCredentials,
  metadata: SessionMetadata,
) -> Result<Session, APIError> {
  let mut conn = match establish_connection() {
    Ok(connection) => connection,
    Err(_) => return Err(APIError::DatabaseError),
  };

  let user_id = match user::get_user_id(&user_credentials.email) {
    Ok(id) => id,
    Err(_) => return Err(APIError::UserNotFound),
  };

  let password_hash = match user::get_password_hash(&user_id) {
    Ok(hash) => hash,
    Err(_) => return Err(APIError::DatabaseError),
  };

  match bcrypt::verify(&user_credentials.password, &password_hash) {
    Ok(valid) => match valid {
      true => (),
      false => return Err(APIError::InvalidPassword),
    },
    Err(_) => return Err(APIError::InternalServerError),
  };

  let session = Session {
    id: Uuid::new_v4().to_string(),
    user_id,
    created_at: util::unix_time::unix_ms(),
    accessed_at: util::unix_time::unix_ms(),
    ip_address: metadata.ip_address,
    user_agent: metadata.user_agent,
  };

  match diesel::insert_into(schema::sessions::table)
    .values(&session)
    .execute(&mut conn)
  {
    Ok(_) => Ok(session),
    Err(_) => Err(APIError::DatabaseError),
  }
}

/// Updates the session metadata (accessed_at, ip_address, user_agent)
async fn update_session(session_id: &str, request: &Request) -> Result<Session, APIError> {
  let mut conn = match establish_connection() {
    Ok(connection) => connection,
    Err(_) => return Err(APIError::DatabaseError),
  };

  let session_metadata = session_metadata(request).await;

  match diesel::update(schema::sessions::table.filter(schema::sessions::id.eq(session_id)))
    .set((
      schema::sessions::accessed_at.eq(util::unix_time::unix_ms()),
      schema::sessions::ip_address.eq(session_metadata.ip_address),
      schema::sessions::user_agent.eq(session_metadata.user_agent),
    ))
    .execute(&mut conn)
  {
    Ok(_) => get_user_session_by_id(session_id),
    Err(_) => Err(APIError::DatabaseError),
  }
}

/// Retrieves a user session by its ID
pub fn get_user_session_by_id(session_id: &str) -> Result<Session, APIError> {
  let mut conn = match establish_connection() {
    Ok(connection) => connection,
    Err(_) => return Err(APIError::DatabaseError),
  };

  match schema::sessions::table
    .filter(schema::sessions::id.eq(session_id))
    .first::<Session>(&mut conn)
  {
    Ok(session) => Ok(session),
    Err(_) => Err(APIError::SessionNotFound),
  }
}

pub fn get_all_user_sessions(user_id: &str) -> Result<Vec<Session>, APIError> {
  let mut conn = match establish_connection() {
    Ok(connection) => connection,
    Err(_) => return Err(APIError::DatabaseError),
  };

  match schema::sessions::table
    .filter(schema::sessions::user_id.eq(&user_id))
    .order(schema::sessions::accessed_at.desc())
    .load::<Session>(&mut conn)
  {
    Ok(sessions) => Ok(sessions),
    Err(_) => Err(APIError::DatabaseError),
  }
}

pub fn delete_user_session(session_id: &str) -> Result<bool, APIError> {
  let mut conn = match establish_connection() {
    Ok(connection) => connection,
    Err(_) => return Err(APIError::DatabaseError),
  };

  match diesel::delete(schema::sessions::table.filter(schema::sessions::id.eq(session_id)))
    .execute(&mut conn)
  {
    Ok(rows_affected) => Ok(rows_affected > 0),
    Err(_) => Err(APIError::DatabaseError),
  }
}

pub fn delete_all_user_sessions(user_id: &str) -> Result<bool, APIError> {
  let mut conn = match establish_connection() {
    Ok(connection) => connection,
    Err(_) => return Err(APIError::DatabaseError),
  };

  match diesel::delete(schema::sessions::table.filter(schema::sessions::user_id.eq(user_id)))
    .execute(&mut conn)
  {
    Ok(rows_affected) => Ok(rows_affected > 0),
    Err(_) => Err(APIError::DatabaseError),
  }
}

use std::time::Instant;

use crate::establish_connection;

#[derive(Debug, serde::Serialize)]
pub struct DatabaseStatus {
  pub connected: bool,
  pub response_time_ms: u128,
}

#[derive(Debug, serde::Serialize)]
pub struct HealthResponse {
  pub status: String,
  pub database: DatabaseStatus,
  pub response_time_ms: u128,
}

pub async fn health_check() -> HealthResponse {
  let now = Instant::now();
  let db_status = match establish_connection() {
    Ok(_) => DatabaseStatus {
      connected: true,
      response_time_ms: now.elapsed().as_millis(),
    },
    Err(_) => DatabaseStatus {
      connected: false,
      response_time_ms: now.elapsed().as_millis(),
    },
  };

  HealthResponse {
    status: "ok".to_string(),
    database: db_status,
    response_time_ms: now.elapsed().as_millis(),
  }
}

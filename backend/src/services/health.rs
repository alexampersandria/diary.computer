use crate::establish_connection;
use dotenvy::dotenv;
use std::{env, time::Instant};

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

  // if total response time exceeds threshold, status should be "degraded"
  // if db_status.connected is false, status should be "critical"
  let mut status = "ok".to_string();
  dotenv().ok();
  const DEFAULT_THRESHOLD: u128 = 100;
  let threshold = match env::var("HEALTH_RESPONSE_TIME_THRESHOLD_MS") {
    Ok(val) => val.parse::<u128>().unwrap_or(DEFAULT_THRESHOLD),
    Err(_) => DEFAULT_THRESHOLD,
  };

  if !db_status.connected {
    status = "critical".to_string();
  } else if now.elapsed().as_millis() > threshold {
    status = "degraded".to_string();
  }

  HealthResponse {
    status,
    database: db_status,
    response_time_ms: now.elapsed().as_millis(),
  }
}

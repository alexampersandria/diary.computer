#![forbid(unsafe_code)]

pub mod api;
pub mod middleware;
pub mod schema;
pub mod services;
pub mod util;

use crate::util::ServiceError;
use diesel::{pg, Connection};
use diesel_migrations::{embed_migrations, MigrationHarness};
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> Result<pg::PgConnection, ServiceError> {
  dotenv().ok();

  let database_url = match env::var("DATABASE_URL") {
    Ok(url) => url,
    Err(_) => {
      tracing::event!(tracing::Level::ERROR, "DATABASE_URL must be set");
      return Err(ServiceError::MigrationError);
    }
  };

  let conn = match pg::PgConnection::establish(&database_url) {
    Ok(connection) => connection,
    Err(e) => {
      tracing::event!(tracing::Level::ERROR, "error connecting to database: {e:?}");
      return Err(ServiceError::MigrationError);
    }
  };

  Ok(conn)
}

pub const EMBEDDED_MIGRATIONS: diesel_migrations::EmbeddedMigrations =
  embed_migrations!("./migrations");

pub fn run_migrations() -> Result<(), ServiceError> {
  tracing::event!(tracing::Level::INFO, "running database migrations...");

  let mut conn = match establish_connection() {
    Ok(connection) => connection,
    Err(e) => {
      tracing::event!(tracing::Level::ERROR, "error connecting to database: {e:?}");
      return Err(ServiceError::MigrationError);
    }
  };

  let pending = match conn.pending_migrations(EMBEDDED_MIGRATIONS) {
    Ok(migrations) => migrations,
    Err(e) => {
      tracing::event!(
        tracing::Level::ERROR,
        "error checking pending migrations: {e}"
      );
      return Err(ServiceError::MigrationError);
    }
  };

  if pending.is_empty() {
    tracing::event!(tracing::Level::INFO, "no pending migrations found");
    return Ok(());
  } else {
    tracing::event!(tracing::Level::INFO, "pending migrations:");
    for migration in pending {
      tracing::event!(tracing::Level::INFO, "+ {}", migration.name());
    }
  }

  return match conn.run_pending_migrations(EMBEDDED_MIGRATIONS) {
    Ok(applied_migrations) => {
      tracing::event!(tracing::Level::INFO, "applied migrations:");
      for migration in applied_migrations {
        tracing::event!(tracing::Level::INFO, "+ {migration}");
      }
      Ok(())
    }
    Err(e) => {
      tracing::event!(tracing::Level::ERROR, "error running migrations: {e}");
      Err(ServiceError::MigrationError)
    }
  };
}

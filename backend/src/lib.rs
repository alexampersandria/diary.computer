#![forbid(unsafe_code)]

pub mod api;
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
      println!("error running migrations: DATABASE_URL not set");
      return Err(ServiceError::MigrationError);
    }
  };

  let conn = match pg::PgConnection::establish(&database_url) {
    Ok(connection) => connection,
    Err(e) => {
      println!("error connecting to database: {e:?}");
      return Err(ServiceError::MigrationError);
    }
  };

  Ok(conn)
}

pub const EMBEDDED_MIGRATIONS: diesel_migrations::EmbeddedMigrations =
  embed_migrations!("./migrations");

pub fn run_migrations() -> Result<(), ServiceError> {
  println!("running database migrations...");

  let mut conn = match establish_connection() {
    Ok(connection) => connection,
    Err(e) => {
      println!("error connecting to database: {e:?}");
      return Err(ServiceError::MigrationError);
    }
  };

  let pending = match conn.pending_migrations(EMBEDDED_MIGRATIONS) {
    Ok(migrations) => migrations,
    Err(e) => {
      println!("error checking pending migrations: {e}");
      return Err(ServiceError::MigrationError);
    }
  };

  if pending.is_empty() {
    println!("no pending migrations found");
    return Ok(());
  } else {
    println!("pending migrations:");
    for migration in pending {
      println!("+ {}", migration.name());
    }
  }

  let migrations = conn.run_pending_migrations(EMBEDDED_MIGRATIONS);
  match migrations {
    Ok(applied_migrations) => {
      println!("applied migrations:");
      for migration in applied_migrations {
        println!("+ {migration}");
      }
      Ok(())
    }
    Err(e) => {
      println!("error running migrations: {e}");
      Err(ServiceError::MigrationError)
    }
  }
}

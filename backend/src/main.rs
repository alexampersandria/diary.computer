use dotenvy::dotenv;
use std::env;
use tracing_subscriber::fmt::format::FmtSpan;

use diary_dot_computer_backend::{api, run_migrations};
use poem::{
  endpoint::StaticFilesEndpoint,
  listener::TcpListener,
  middleware::{Cors, NormalizePath, TrailingSlash},
  EndpointExt, Route, Server,
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
  dotenv().ok();
  run_migrations().ok();

  let port = env::var("PORT").unwrap_or("3000".to_string());
  let environment = env::var("ENVIRONMENT").unwrap_or("development".to_string());

  // #TODO: remove this when docker is confirmed working
  if environment == "development" {
    println!("debug env vars set/null: ");
    // DATABASE_URL
    match env::var("DATABASE_URL") {
      Ok(_) => println!("DATABASE_URL: SET"),
      Err(_) => println!("DATABASE_URL: NOT SET"),
    }
    // INVITE_REQUIRED
    match env::var("INVITE_REQUIRED") {
      Ok(_) => println!("INVITE_REQUIRED: SET"),
      Err(_) => println!("INVITE_REQUIRED: NOT SET"),
    }
    // PORT
    match env::var("PORT") {
      Ok(_) => println!("PORT: SET"),
      Err(_) => println!("PORT: NOT SET"),
    }
    // ENVIRONMENT
    match env::var("ENVIRONMENT") {
      Ok(_) => println!("ENVIRONMENT: SET"),
      Err(_) => println!("ENVIRONMENT: NOT SET"),
    }
    // BCRYPT_COST
    match env::var("BCRYPT_COST") {
      Ok(_) => println!("BCRYPT_COST: SET"),
      Err(_) => println!("BCRYPT_COST: NOT SET"),
    }
  }

  // no allow_origin means all origins are allowed, dev allows all
  let dev_cors = Cors::new();
  // #TODO: restrict in production
  // let prod_cors = Cors::new().allow_origin(url);
  let prod_cors = Cors::new();

  let cors = if environment == "development" {
    dev_cors
  } else {
    prod_cors
  };

  println!("starting server in {environment} mode");

  tracing_subscriber::fmt()
    .with_span_events(FmtSpan::FULL)
    .init();

  use poem::middleware::Tracing;

  let app = Route::new()
    .nest("/api", api::index::endpoint())
    .nest(
      "/",
      StaticFilesEndpoint::new("../www")
        .index_file("index.html")
        // fallback to index for any non-static built items
        // this allows for sveltekit routing to take over
        .fallback_to_index(),
    )
    .with(NormalizePath::new(TrailingSlash::Trim))
    .with(cors)
    .with(Tracing);

  println!("listening on port {port}");

  Server::new(TcpListener::bind(format!("0.0.0.0:{port}")))
    .run(app)
    .await
}

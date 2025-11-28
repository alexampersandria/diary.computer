use diarycomputer::{api, middleware, run_migrations};
use dotenvy::dotenv;
use poem::{
  endpoint::StaticFilesEndpoint,
  listener::TcpListener,
  middleware::{Cors, NormalizePath, TrailingSlash},
  EndpointExt, Route, Server,
};
use std::env;
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
  dotenv().ok();

  let port = env::var("PORT").unwrap_or("3000".to_string());
  let environment = env::var("ENVIRONMENT").unwrap_or("development".to_string());

  tracing_subscriber::fmt()
    .log_internal_errors(true)
    .with_max_level(match environment.as_str() {
      "development" => tracing::Level::DEBUG,
      "production" => tracing::Level::INFO,
      _ => tracing::Level::INFO,
    })
    .with_span_events(FmtSpan::FULL)
    .init();

  tracing::event!(tracing::Level::INFO, "📔 diary.computer");
  tracing::event!(
    tracing::Level::INFO,
    "starting server in {environment} mode"
  );

  // #TODO: remove this when docker is confirmed working
  if environment == "development" {
    // DATABASE_URL
    match env::var("DATABASE_URL") {
      Ok(_) => tracing::event!(tracing::Level::DEBUG, "DATABASE_URL: SET"),
      Err(_) => tracing::event!(tracing::Level::DEBUG, "DATABASE_URL: NOT SET"),
    }
    // INVITE_REQUIRED
    match env::var("INVITE_REQUIRED") {
      Ok(_) => tracing::event!(tracing::Level::DEBUG, "INVITE_REQUIRED: SET"),
      Err(_) => tracing::event!(tracing::Level::DEBUG, "INVITE_REQUIRED: NOT SET"),
    }
    // PORT
    match env::var("PORT") {
      Ok(_) => tracing::event!(tracing::Level::DEBUG, "PORT: SET"),
      Err(_) => tracing::event!(tracing::Level::DEBUG, "PORT: NOT SET"),
    }
    // ENVIRONMENT
    match env::var("ENVIRONMENT") {
      Ok(_) => tracing::event!(tracing::Level::DEBUG, "ENVIRONMENT: SET"),
      Err(_) => tracing::event!(tracing::Level::DEBUG, "ENVIRONMENT: NOT SET"),
    }
    // BCRYPT_COST
    match env::var("BCRYPT_COST") {
      Ok(_) => tracing::event!(tracing::Level::DEBUG, "BCRYPT_COST: SET"),
      Err(_) => tracing::event!(tracing::Level::DEBUG, "BCRYPT_COST: NOT SET"),
    }
  }

  run_migrations().ok();

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
    .with(middleware::Trace)
    .with(cors);

  tracing::event!(tracing::Level::INFO, "listening on port {port}");

  Server::new(TcpListener::bind(format!("0.0.0.0:{port}")))
    .run(app)
    .await
}

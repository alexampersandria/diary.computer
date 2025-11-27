use dotenvy::dotenv;
use std::env;
use tracing_subscriber::fmt::format::FmtSpan;

use diary_dot_computer_backend::api;
use poem::{
  endpoint::StaticFilesEndpoint,
  listener::TcpListener,
  middleware::{Cors, NormalizePath, TrailingSlash},
  EndpointExt, Route, Server,
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
  dotenv().ok();

  let port = env::var("PORT").unwrap_or("3000".to_string());
  let environment = env::var("ENVIRONMENT").unwrap_or("development".to_string());
  let url = env::var("URL").unwrap_or(format!("http://localhost:{port}"));

  let dev_cors = Cors::new()
    .allow_origin("http://localhost:5173")
    .allow_origin("http://127.0.0.1:5173")
    .allow_origin(format!("http://localhost:{port}"))
    .allow_origin(format!("http://127.0.0.1:{port}"))
    .allow_origin(format!("http://0.0.0.0:{port}"));
  let prod_cors = Cors::new()
    .allow_origin(url)
    .allow_origin(format!("http://0.0.0.0:{port}"));

  // #TODO: actually use CORS
  let _cors = if environment == "development" {
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
    .with(Tracing);

  println!("listening on port {port}");

  Server::new(TcpListener::bind(format!("0.0.0.0:{port}")))
    .run(app)
    .await
}

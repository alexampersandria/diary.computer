use crate::{api::v1, util::response::response};
use poem::{get, handler, http::StatusCode, Response, Route};
use serde::Serialize;

#[derive(Serialize)]
pub struct PackageInfo {
  pub name: String,
  pub version: String,
}

#[handler]
pub fn package_version() -> Response {
  let package_info = PackageInfo {
    name: env!("CARGO_PKG_NAME").into(),
    version: env!("CARGO_PKG_VERSION").into(),
  };

  response(StatusCode::OK, &package_info)
}

pub fn v1_endpoint() -> poem::Route {
  Route::new().nest("/", v1::index::endpoint())
}

pub fn endpoint() -> poem::Route {
  Route::new()
    .at("/", get(package_version))
    .nest("/v1", v1_endpoint())
}

use std::time::Instant;

use tracing::Level;

use poem::{
  web::RealIp, Endpoint, FromRequest, IntoResponse, Middleware, Request, Response, Result,
};

/// custom implementation of the poem tracing middleware
/// original https://github.com/poem-web/poem/blob/master/poem/src/middleware/tracing_mw.rs
/// this implementation uses evet logging instead of spans for fewer lines in the logs
/// as well as warns on non-2XX responses
#[derive(Default)]
pub struct Trace;

impl<E: Endpoint> Middleware<E> for Trace {
  type Output = TraceEndpoint<E>;

  fn transform(&self, ep: E) -> Self::Output {
    TraceEndpoint { inner: ep }
  }
}

pub struct TraceEndpoint<E> {
  inner: E,
}

impl<E: Endpoint> Endpoint for TraceEndpoint<E> {
  type Output = Response;

  async fn call(&self, req: Request) -> Result<Self::Output> {
    let remote_addr = RealIp::from_request_without_body(&req)
      .await
      .ok()
      .and_then(|real_ip| real_ip.0)
      .map(|addr| addr.to_string())
      .unwrap_or_else(|| req.remote_addr().to_string());

    let method = req.method().to_string();
    let uri = req.uri().to_string();

    async move {
      let now = Instant::now();
      let res = self.inner.call(req).await;
      let duration = now.elapsed();

      match res {
        Ok(resp) => {
          let resp = resp.into_response();
          let status = resp.status();
          let status_code = status.as_u16();

          if status.is_success() {
            if uri.starts_with("/api") {
              tracing::event!(
                Level::INFO,
                "{} {} {} ({:?}) from {}",
                method,
                uri,
                status_code,
                duration,
                remote_addr,
              );
            } else {
              tracing::event!(
                Level::DEBUG,
                "{} {} {} ({:?}) from {}",
                method,
                uri,
                status_code,
                duration,
                remote_addr,
              );
            }
          } else {
            tracing::event!(
              Level::WARN,
              "{} {} {} ({:?}) from {}",
              method,
              uri,
              status_code,
              duration,
              remote_addr,
            );
          }
          Ok(resp)
        }
        Err(err) => {
          let status_code = err.status().as_u16();
          tracing::event!(
            Level::ERROR,
            "{} {} {} ({:?}) from {}",
            method,
            uri,
            status_code,
            duration,
            remote_addr,
          );
          Err(err)
        }
      }
    }
    .await
  }
}

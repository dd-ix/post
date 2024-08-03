use std::future::IntoFuture;
use std::iter::once;
use std::sync::Arc;

use axum::http::header::AUTHORIZATION;
use clap::Parser;
use tokio::net::TcpListener;
use tokio::select;
use tower_http::sensitive_headers::SetSensitiveRequestHeadersLayer;
use tower_http::trace::TraceLayer;
use tower_http::validate_request::ValidateRequestHeaderLayer;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use utils::shutdown_signal;

use crate::args::Args;
use crate::ctx::Context;
use crate::routes::router;
use crate::service::{EmailService, TemplateService};

mod args;
mod ctx;
mod routes;
mod service;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let args = Args::parse();

  let subscriber = FmtSubscriber::builder()
    .with_max_level(Level::INFO)
    .compact()
    .finish();

  tracing::subscriber::set_global_default(subscriber)?;

  info!(concat!(
    "booting ",
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    "..."
  ));

  let listener = TcpListener::bind(args.listen_addr).await?;
  info!("listening at http://{}...", args.listen_addr);

  let email_service = Arc::new(EmailService::new(args.smtp_addr));
  let template_service = Arc::new(TemplateService::new(&args.template_glob)?);

  let router = router()
    .with_state(Context {
      email_service,
      template_service,
    })
    .layer(ValidateRequestHeaderLayer::bearer(&match args.api_token {
      None => tokio::fs::read_to_string(args.api_token_file.unwrap()).await?,
      Some(token) => token,
    }))
    .layer(SetSensitiveRequestHeadersLayer::new(once(AUTHORIZATION)))
    .layer(TraceLayer::new_for_http())
    .into_make_service();

  let axum = axum::serve(listener, router)
    .with_graceful_shutdown(shutdown_signal())
    .into_future();

  select! {
    result = axum => { result? }
  }

  Ok(())
}

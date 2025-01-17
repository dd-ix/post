use axum::routing::post;
use axum::Router;

use crate::ctx::Context;

mod send;

pub(super) fn router() -> Router<Context> {
  Router::new()
    .route("/v1/send", post(send::adhoc))
    .route("/v1/send/{template_name}", post(send::template))
}

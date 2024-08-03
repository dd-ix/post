use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use lettre::Message;
use lettre::message::header::ContentType;
use lettre::message::MessageBuilder;
use serde::Deserialize;
use tera::Value;
use tracing::error;
use tracing::warn;

use crate::ctx::Context;

#[derive(Deserialize, Clone)]
pub(super) struct Mailbox {
  name: Option<String>,
  email: String,
}

#[derive(Deserialize)]
pub(super) struct AdhocRequest {
  from: Mailbox,
  replay_to: Option<Mailbox>,
  to: Vec<Mailbox>,
  cc: Vec<Mailbox>,
  bcc: Vec<Mailbox>,
  subject: String,
  body: String,
}

pub(super) async fn adhoc(
  State(ctx): State<Context>,
  Json(req): Json<AdhocRequest>,
) -> Result<StatusCode, StatusCode> {
  let message = req.try_into().map_err(|err| {
    warn!("unable to parse message: {:?}", err);
    StatusCode::BAD_REQUEST
  })?;

  ctx.email_service.send(message).await.map_err(|err| {
    error!("unable to send mail: {:?}", err);
    StatusCode::INTERNAL_SERVER_ERROR
  })?;

  Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
pub(super) struct TemplateRequest {
  from: Mailbox,
  replay_to: Option<Mailbox>,
  to: Vec<Mailbox>,
  cc: Vec<Mailbox>,
  bcc: Vec<Mailbox>,
  subject: String,
  data: Value,
}

pub(super) async fn template(
  State(ctx): State<Context>,
  Path(template_name): Path<String>,
  Json(req): Json<TemplateRequest>,
) -> Result<StatusCode, StatusCode> {
  let message_builder = create_builder(&req).map_err(|err| {
    warn!("unable to parse message: {:?}", err);
    StatusCode::BAD_REQUEST
  })?;

  let body = ctx
    .template_service
    .render(&template_name, req.data)
    .map_err(|err| {
      warn!("unable to render template: {:?}", err);
      StatusCode::BAD_REQUEST
    })?;

  let message = message_builder
    .header(ContentType::TEXT_HTML)
    .body(body)
    .map_err(|err| {
      warn!("unable to parse message: {:?}", err);
      StatusCode::BAD_REQUEST
    })?;

  ctx
    .email_service
    .send(message)
    .await
    .map_err(|err| {
      error!("unable to send mail: {:?}", err);
      StatusCode::INTERNAL_SERVER_ERROR
    })?;

  Ok(StatusCode::NO_CONTENT)
}

impl TryFrom<Mailbox> for lettre::message::Mailbox {
  type Error = anyhow::Error;

  fn try_from(value: Mailbox) -> Result<Self, Self::Error> {
    Ok(Self {
      name: value.name,
      email: value.email.parse()?,
    })
  }
}

impl TryFrom<AdhocRequest> for Message {
  type Error = anyhow::Error;

  fn try_from(req: AdhocRequest) -> Result<Self, Self::Error> {
    Ok({
      let mut builder = Message::builder()
        .from(req.from.try_into()?)
        .subject(req.subject);

      if let Some(replay_to) = req.replay_to {
        builder = builder.reply_to(replay_to.try_into()?);
      }

      for to in req.to {
        builder = builder.to(to.try_into()?);
      }

      for cc in req.cc {
        builder = builder.cc(cc.try_into()?);
      }

      for bcc in req.bcc {
        builder = builder.bcc(bcc.try_into()?);
      }

      builder.body(req.body)?
    })
  }
}

fn create_builder(req: &TemplateRequest) -> anyhow::Result<MessageBuilder> {
  let mut builder = Message::builder()
    .from(req.from.clone().try_into()?)
    .subject(req.subject.clone());

  if let Some( replay_to) = &req.replay_to {
    builder = builder.reply_to(replay_to.clone().try_into()?);
  }

  for to in req.to.clone() {
    builder = builder.to(to.try_into()?);
  }

  for cc in req.cc .clone(){
    builder = builder.cc(cc.try_into()?);
  }

  for bcc in req.bcc.clone() {
    builder = builder.bcc(bcc.try_into()?);
  }

  Ok(builder)
}
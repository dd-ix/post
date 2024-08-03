use std::net::SocketAddr;

use lettre::transport::smtp::response::Response;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

pub(crate) struct EmailService {
  mailer: AsyncSmtpTransport<Tokio1Executor>,
}

impl EmailService {
  pub(crate) fn new(smtp_host: SocketAddr) -> Self {
    Self {
      mailer: AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(smtp_host.ip().to_string())
        .port(smtp_host.port())
        .build(),
    }
  }

  pub(crate) async fn send(&self, email: Message) -> anyhow::Result<Response> {
    Ok(self.mailer.send(email).await?)
  }
}

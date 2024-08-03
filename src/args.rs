use std::net::SocketAddr;

use clap::Parser;

#[derive(Parser)]
#[clap(about, version)]
pub(super) struct Args {
  #[arg(short, long, env = "POST_LISTEN_ADDR", default_value = "[::]:9876")]
  pub(super) listen_addr: SocketAddr,
  #[arg(short = 'a', long, env = "POST_SMTP_ADDR", default_value = "[::1]:25")]
  pub(super) smtp_addr: SocketAddr,
  #[arg(short = 't', long, env = "POST_TEMPLATE_GLOB")]
  pub(super) template_glob: String,
}

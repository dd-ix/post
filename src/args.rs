use std::{net::SocketAddr, path::PathBuf};

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
  #[arg(
    short = 's',
    long,
    env = "POST_API_TOKEN",
    conflicts_with = "api_token_file",
    required_unless_present = "api_token_file"
  )]
  pub(super) api_token: Option<String>,
  #[arg(
    short = 'f',
    long,
    env = "POST_API_TOKEN_FILE",
    conflicts_with = "api_token",
    required_unless_present = "api_token"
  )]
  pub(super) api_token_file: Option<PathBuf>,
}

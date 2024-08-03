use std::sync::Arc;

use crate::service::{EmailService, TemplateService};

#[derive(Clone)]
pub(crate) struct Context {
  pub(crate) email_service: Arc<EmailService>,
  pub(crate) template_service: Arc<TemplateService>,
}

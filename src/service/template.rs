use tera::{Context, Tera, Value};

pub(crate) struct TemplateService {
  tera: Tera,
}

impl TemplateService {
  pub(crate) fn new(template_glob: &str) -> anyhow::Result<Self> {
    Ok(Self {
      tera: Tera::new(template_glob)?,
    })
  }

  pub(crate) fn render(&self, name: &str, params: Value) -> anyhow::Result<String> {
    let ctx = Context::from_value(params)?;
    Ok(self.tera.render(name, &ctx)?)
  }
}

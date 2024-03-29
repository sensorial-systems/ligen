use crate::prelude::*;

use handlebars;
use handlebars::{Handlebars, Context, Helper, HelperResult, Output, RenderContext};

#[derive(Default)]
pub struct Template {
    handlebars: Handlebars<'static>
}

impl Template {
    pub fn render<S, T>(&self, template: S, value: T) -> Result<String>
    where T: Serialize, S: AsRef<str> {
        let value = serde_json::to_value(value)?;
        let result = self
            .handlebars
            .render(template.as_ref(), &value)
            .map_err(|e| format!("Failed to render template: {}", e))?;
        Ok(result)
    }
}

pub struct Inputs {
    params: Vec<serde_json::Value>
}

impl Inputs {
    pub fn get(&self, index: usize) -> Option<serde_json::Value> {
        self
            .params
            .get(index)
            .filter(|value| !value.is_null())
            .cloned()
    }
}

impl From<&Helper<'_, '_>> for Inputs {
    fn from(helper: &Helper<'_, '_>) -> Self {
        let params = helper
            .params()
            .iter()
            .map(|param| param.value())
            .cloned()
            .collect::<Vec<_>>();
        Self { params }
    }
}

impl Template {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn register_template<S: AsRef<str>>(&mut self, name: S, content: S) -> Result<()> {
        self
            .handlebars
            .register_template_string(name.as_ref(), content.as_ref())
            .map_err(|e| Error::Message(format!("Failed to register template: {}", e)))
    }

    pub fn register_function<S: AsRef<str>, F: Fn(&Inputs) -> String + Send + Sync + 'static>(&mut self, name: S, function: F) {
        let function = Box::new(function);
        self.handlebars.register_helper(name.as_ref(), Box::new(move |helper: &Helper<'_, '_>, _: &Handlebars<'_>, _context: &Context, _rc: &mut RenderContext<'_, '_>, out: &mut dyn Output| -> HelperResult {
            let inputs = Inputs::from(helper);
            let output = (*function)(&inputs);
            out.write(&output).ok();
            Ok(())
        }));
    }
}
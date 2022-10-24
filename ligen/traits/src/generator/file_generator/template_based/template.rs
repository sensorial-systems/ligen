use handlebars;
use handlebars::{Handlebars, Context, Helper, HelperResult, Output, RenderContext};

#[derive(Default)]
pub struct Template {
    handlebars: Handlebars<'static>
}

impl Template {
    pub fn render<S: AsRef<str>>(&self, template: S, value: &serde_json::Value) -> Result<String, Box<dyn std::error::Error>> {
        Ok(self.handlebars.render(template.as_ref(), value)?)
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
            .map(|param| param.clone())
            .collect::<Vec<_>>();
        Self { params }
    }
}

impl Template {
    pub(crate) fn new() -> Self {
        Default::default()
    }

    pub fn register_template<S: AsRef<str>>(&mut self, name: S, content: S) {
        self.handlebars.register_template_string(name.as_ref(), content.as_ref()).ok();
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
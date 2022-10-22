use ligen_ir::*;

use ligen_traits::generator::{FileSet, handlebars, TemplateBasedGenerator, TemplateSetup};
use std::path::PathBuf;
use std::str::FromStr;

use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext};
use ligen_traits::prelude::*;
use ligen_traits::templates;

// TODO: #[derive(ligen::Template)) to automatically fetch templates.
#[derive(Debug, Default)]
pub struct CGenerator;

impl TemplateSetup for CGenerator {
    fn get_template(&self) -> Result<Handlebars> {
        Ok(templates!(identifier, arguments, implementation, method, function, module, object, parameters, project))
    }
}

impl TemplateBasedGenerator for CGenerator {
    fn get_functions(&self, _project: &Project, template: &mut Handlebars) {
        template.register_helper("marshal_type", Box::new(move |h: &Helper<'_, '_>, _: &Handlebars<'_>, _context: &Context, _rc: &mut RenderContext<'_, '_>, out: &mut dyn Output| -> HelperResult {
            let param = h
                .param(0)
                .map(|value| value.value().clone())
                .filter(|value| !value.is_null());
            let content = if let Some(_param) = param {
                // let type_ = serde_json::from_value::<Type>(param).unwrap();
                // let identifier = type_.path().last();
                "int".to_string()
            } else {
                "void".to_string()
            };
            out.write(&content)?;
            Ok(())
        }));
        template.register_helper("join_path", Box::new(move |h: &Helper<'_, '_>, _: &Handlebars<'_>, _context: &Context, _rc: &mut RenderContext<'_, '_>, out: &mut dyn Output| -> HelperResult {
            let separator = serde_json::from_value::<String>(h.param(0).unwrap().value().clone()).unwrap();
            let path = serde_json::from_value::<Path>(h.param(1).unwrap().value().clone()).unwrap();
            let content = path.to_string(&separator);
            out.write(&content)?;
            Ok(())
        }));
    }

    fn base_path(&self) -> PathBuf {
        PathBuf::from("c".to_string())
    }

    fn generate_module(&self, project: &Project, module: &Module, file_set: &mut FileSet, template: &Handlebars) -> Result<()> {
        let value = serde_json::to_value(&module)?;
        let content = template.render("module", &value).map_err(|e| Error::Message(format!("{}", e)))?;
        let mut path = PathBuf::from_str("include").unwrap();
        for segment in module.path.clone().segments {
            path = path.join(segment.name);
        }
        path = path.with_extension("h");
        file_set.entry(&path).writeln(content);
        for module in &module.modules {
            self.generate_module(project, module, file_set, template)?;
        }
        Ok(())
    }}

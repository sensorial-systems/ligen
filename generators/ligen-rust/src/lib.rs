pub mod prelude;
pub mod parsing;

extern crate proc_macro;

use ligen_ir::*;

use ligen_traits::generator::file_generator::{FileSet, TemplateBasedGenerator, handlebars, TemplateSetup};
use std::path::PathBuf;
use std::str::FromStr;
use ligen_ir::Type;

use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext};
use ligen_traits::prelude::*;
use ligen_traits::templates;

#[derive(Debug, Default)]
pub struct RustGenerator;

impl TemplateSetup for RustGenerator {
    fn get_template(&self) -> Result<Handlebars> {
        Ok(templates!(identifier, arguments, implementation, method, function, module, object, parameters, project))
    }
}

impl TemplateBasedGenerator for RustGenerator {
    fn get_functions(&self, project: &Project, template: &mut Handlebars) {
        let root_module = project.root_module.clone();
        template.register_helper("marshal_type", Box::new(move |h: &Helper<'_, '_>, _: &Handlebars<'_>, _context: &Context, _rc: &mut RenderContext<'_, '_>, out: &mut dyn Output| -> HelperResult {
            let param = h
                .param(0)
                .map(|value| value.value().clone())
                .filter(|value| !value.is_null());
            let content = if let Some(param) = param {
                let type_ = serde_json::from_value::<Type>(param).unwrap();
                let identifier = type_.path().last();
                let is_opaque = root_module
                    .get_literal_from_path(format!("ligen::ffi::{}::opaque", identifier.name))
                    .map(|literal| literal.to_string() == "true")
                    .unwrap_or_default();
                let (type_, opacity) = if is_opaque {
                    (type_.drop_reference().to_string(), "*mut ")
                } else {
                    (type_.to_string(), "")
                };
                format!("{}{}", opacity, type_)
            } else {
                format!("()")
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
        template.register_helper("get_name", Box::new(move |h: &Helper<'_, '_>, _: &Handlebars<'_>, _context: &Context, _rc: &mut RenderContext<'_, '_>, out: &mut dyn Output| -> HelperResult {
            let path = serde_json::from_value::<Path>(h.param(0).unwrap().value().clone()).unwrap();
            let content = path.last();
            out.write(&content.name)?;
            Ok(())
        }));
    }

    fn base_path(&self) -> PathBuf {
        PathBuf::from("rust".to_string())
    }

    fn generate_module(&self, project: &Project, module: &Module, file_set: &mut FileSet, template: &Handlebars) -> Result<()> {
        let is_root_module = project.root_module.path == module.path;
        let name = if is_root_module { "lib.rs" } else { "mod.rs" };
        let value = serde_json::to_value(&module)?;
        let content = template.render("module", &value).map_err(|e| Error::Message(format!("{}", e)))?;
        let mut path = PathBuf::from_str("src").unwrap();
        for segment in module.path.clone().without_first().segments {
            path = path.join(segment.name);
        }
        path = path.join(name);
        file_set.entry(&path).writeln(content);
        for module in &module.modules {
            self.generate_module(project, module, file_set, template)?;
        }
        Ok(())
    }
}

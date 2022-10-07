pub mod prelude;
pub mod parsing;

extern crate proc_macro;

use ligen_ir::*;

use ligen_traits::generator::{ProjectVisitor, Generator, FileSet, FileGenerator};
use std::path::PathBuf;
use std::str::FromStr;
use ligen_ir::Type;

use handlebars::{Context, Handlebars as Template, Handlebars, Helper, HelperResult, Output, RenderContext};
use ligen_traits::prelude::{Error, Result as LigenResult};

#[derive(Debug, Default)]
pub struct RustGenerator;

impl RustGenerator {
    pub fn get_template(&self) -> LigenResult<Template> {
        let mut template = Template::new();
        template.register_template_string("identifier", include_str!("templates/identifier.hbs")).expect("Failed to load identifier template.");
        template.register_template_string("arguments", include_str!("templates/arguments.hbs")).expect("Failed to load arguments template.");
        template.register_template_string("implementation", include_str!("templates/implementation.hbs")).expect("Failed to load implementation template.");
        template.register_template_string("import", include_str!("templates/import.hbs")).expect("Failed to load import template.");
        template.register_template_string("method", include_str!("templates/method.hbs")).expect("Failed to load method template.");
        template.register_template_string("module", include_str!("templates/module.hbs")).expect("Failed to load module template.");
        template.register_template_string("object", include_str!("templates/object.hbs")).expect("Failed to load object template.");
        template.register_template_string("parameters", include_str!("templates/parameters.hbs")).expect("Failed to load parameters template.");
        template.register_template_string("project", include_str!("templates/project.hbs")).expect("Failed to load project template.");
        Ok(template)
    }

    pub fn get_functions(&self, template: &mut Template, visitor: &ProjectVisitor) {
        let root_module = visitor.current.root_module.clone();
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
                    ("*const u8".to_string(), "")
                };
                format!("{}{}", opacity, type_)
            } else {
                format!("()")
            };
            out.write(&content)?;
            Ok(())
        }));
    }
}

impl Generator for RustGenerator {
    fn base_path(&self) -> PathBuf {
        PathBuf::from("rust".to_string())
    }
}

impl FileGenerator for RustGenerator {
    fn generate_files(&self, file_set: &mut FileSet, visitor: &ProjectVisitor) -> LigenResult<()> {
        let mut template = self.get_template()?;
        self.get_functions(&mut template, visitor);
        let value = serde_json::to_value(&visitor.current)?;
        let content = template.render("project", &value).map_err(|e| Error::Message(format!("{}", e)))?;
        let file = file_set.entry(&PathBuf::from_str("src").unwrap().join("lib.rs"));
        file.writeln(content);
        Ok(())
    }
}

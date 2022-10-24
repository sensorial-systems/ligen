pub mod prelude;
pub mod parsing;

extern crate proc_macro;

use ligen_ir::*;

use ligen_traits::generator::file_generator::{TemplateBasedGenerator, TemplateRegister, Template};
use std::path::PathBuf;
use std::str::FromStr;
use ligen_ir::Type;

use ligen_traits::prelude::*;
use ligen_traits::register_templates;

#[derive(Debug, Default)]
pub struct RustGenerator;

impl TemplateRegister for RustGenerator {
    fn register_templates(&self, template: &mut Template) -> Result<()> {
        register_templates!(template, identifier, arguments, implementation, method, function, module, object, parameters, project);
        Ok(())
    }
}

impl TemplateBasedGenerator for RustGenerator {
    fn register_functions(&self, project: &Project, template: &mut Template) {
        let root_module = project.root_module.clone();
        template.register_function("marshal_type", move |inputs| {
            if let Some(param) = inputs.get(0) {
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
            }
        });
    }

    fn base_path(&self) -> PathBuf {
        PathBuf::from("rust".to_string())
    }

    fn module_generation_path(&self, project: &Project, module: &Module) -> PathBuf {
        let is_root_module = project.root_module.path == module.path;
        let name = if is_root_module { "lib.rs" } else { "mod.rs" };
        let mut path = PathBuf::from_str("src").unwrap();
        path = path.join(PathBuf::from(module.path.clone().without_first()));
        path = path.join(name);
        path
    }
}

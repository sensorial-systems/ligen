use ligen_ir::*;

use ligen_traits::generator::file_generator::{FileSet, Inputs, Template, TemplateBasedGenerator, TemplateRegister};
use std::path::PathBuf;
use std::str::FromStr;

use ligen_traits::prelude::*;
use ligen_traits::{register_functions, register_templates};

#[derive(Debug, Default)]
pub struct CGenerator;

impl TemplateRegister for CGenerator {
    fn register_templates(&self, template: &mut Template) -> Result<()> {
        register_templates!(template, identifier, arguments, implementation, method, function, module, object, parameters, project);
        Ok(())
    }
}

fn marshal_type(inputs: &Inputs) -> String {
    let param = inputs.get(0);
    if let Some(_param) = param {
        // let type_ = serde_json::from_value::<Type>(param).unwrap();
        // let identifier = type_.path().last();
        "int"
    } else {
        "void"
    }.into()
}

fn join_path(inputs: &Inputs) -> String {
    let separator = serde_json::from_value::<String>(inputs.get(0).unwrap()).unwrap();
    let path = serde_json::from_value::<Path>(inputs.get(1).unwrap()).unwrap();
    path.to_string(&separator)
}

impl TemplateBasedGenerator for CGenerator {
    fn register_functions(&self, _project: &Project, template: &mut Template) {
        register_functions!(template, marshal_type, join_path);
    }

    fn base_path(&self) -> PathBuf {
        PathBuf::from("c".to_string())
    }

    fn generate_module(&self, project: &Project, module: &Module, file_set: &mut FileSet, template: &Template) -> Result<()> {
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

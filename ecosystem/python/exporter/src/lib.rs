pub mod prelude;
use std::{path::PathBuf, str::FromStr};

use ligen_ir::{Module, Library};
use prelude::*;

use ligen_traits::generator::file_generator::{TemplateRegister, Template, TemplateBasedGenerator};


#[derive(Debug, Default)]
pub struct PythonGenerator {}

impl TemplateRegister for PythonGenerator {
    fn register_templates(&self, _template: &mut Template) -> Result<()> {
        // register_templates!(template, identifier, arguments, implementation, method, function, module, object, parameters, library);
        Ok(())
    }
}

impl TemplateBasedGenerator for PythonGenerator {
    fn register_functions(&self, _library: &Library, _template: &mut Template) {
        //register_functions!(template, mapped_type, marshal_output);
    }

    fn base_path(&self) -> PathBuf {
        PathBuf::from("python".to_string())
    }

    fn module_generation_path(&self, _library: &Library, _module: &Module) -> PathBuf {
        // let is_root_module = library.root_module == *module;
        // let name = if is_root_module { "lib.rs" } else { "mod.rs" };
        let path = PathBuf::from_str("src").unwrap();
        // path = path.join(PathBuf::from(module.path.clone().without_first()));
        // path = path.join(name);
        // FIXME: This is not working.
        println!("{}", path.display());
        path
    }
}

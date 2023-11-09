mod template;
mod builtin_functions;

pub use template::*;
// use builtin_functions::*;
// use super::{FileGenerator, FileSet};

// use ligen_ir::{Module, Library};
// use ligen_common::*;

// use std::path::PathBuf;

// #[macro_export]
// macro_rules! register_template {
//     ($template:ident, $identifier:ident) => {
//         $template.register_template(stringify!($identifier), include_str!(concat!("templates/", stringify!($identifier), ".hbs")));
//     }
// }

// TODO: Use #[derive(TemplateFiles)] when https://github.com/rust-lang/rust/issues/54725 is stable.
//  The idea is to iterate over the templates folder and automatically register all the template files.
// #[macro_export]
// macro_rules! register_templates {
//     ($template:ident, $($identifier:ident),+) => {
//         {
//             $($crate::register_template!($template, $identifier);)+
//         }
//     }
// }

// #[macro_export]
// macro_rules! register_functions {
//     ($template:ident, $($identifier:ident),+) => {
//         {
//             $($template.register_function(stringify!($identifier), $identifier);)+
//         }
//     }
// }

// pub trait TemplateRegister {
//     fn register_templates(&self, template: &mut Template) -> Result<()>;
// }

// pub trait TemplateBasedGenerator: TemplateRegister {
//     type Input;
//     fn register_functions(&self, input: &Self::Input, _template: &mut Template) {}

//     fn base_path(&self) -> PathBuf;

//     fn module_generation_path(&self, input: &Self::Input, module: &Module) -> PathBuf;

//     fn generate_module(&self, library: &Library, module: &Module, file_set: &mut FileSet, template: &Template) -> Result<()> {
//         let value = serde_json::to_value(module)?;
//         let content = template.render("module", &value)?;

//         let path = self.module_generation_path(library, module);

//         file_set.entry(&path).section("root").writeln(content);
//         for module in &module.modules {
//             self.generate_module(library, module, file_set, template)?;
//         }
//         Ok(())
//     }
// }

// impl <T: TemplateBasedGenerator> FileGenerator<Input  for T {
//     fn base_path(&self) -> PathBuf {
//         <Self as TemplateBasedGenerator>::base_path(self)
//     }

//     fn generate_files(&self, library: &Library, file_set: &mut FileSet) -> Result<()> {
//         let mut template = Template::new();
//         self.register_templates(&mut template)?;
//         register_functions!(template, name_from_path, join_path, json);
//         self.register_functions(library, &mut template);
//         self.generate_module(library, &library.root_module, file_set, &template)?;
//         Ok(())
//     }
// }
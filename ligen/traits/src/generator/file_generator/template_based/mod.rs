mod template;
mod builtin_functions;

pub use template::*;
use builtin_functions::*;
use super::{FileGenerator, FileSet};

use ligen_ir::{Module, Project};
use ligen_common::*;

use std::path::PathBuf;

#[macro_export]
macro_rules! register_template {
    ($template:ident, $identifier:ident) => {
        // TODO: Stop using expect and use ? instead?
        // $template.register_template_string(stringify!($identifier), include_str!(concat!("templates/", stringify!($identifier), ".hbs"))).expect(concat!("Failed to load ", stringify!($identifier), " template."));
        $template.register_template(stringify!($identifier), include_str!(concat!("templates/", stringify!($identifier), ".hbs")));
    }
}

// TODO: Use #[derive(TemplateFiles)] when https://github.com/rust-lang/rust/issues/54725 is stable.
//  The idea is to iterate over the templates folder and automatically register all the template files.
#[macro_export]
macro_rules! register_templates {
    ($template:ident, $($identifier:ident),+) => {
        {
            $($crate::register_template!($template, $identifier);)+
        }
    }
}

#[macro_export]
macro_rules! register_functions {
    ($template:ident, $($identifier:ident),+) => {
        {
            $($template.register_function(stringify!($identifier), $identifier);)+
        }
    }
}

pub trait TemplateRegister {
    fn register_templates(&self, template: &mut Template) -> Result<()>;
}

pub trait TemplateBasedGenerator: TemplateRegister {
    fn register_functions(&self, project: &Project, template: &mut Template);

    fn base_path(&self) -> PathBuf;

    fn generate_module(&self, project: &Project, module: &Module, file_set: &mut FileSet, template: &Template) -> Result<()>;
}

impl <T: TemplateBasedGenerator> FileGenerator for T {
    fn base_path(&self) -> PathBuf {
        <Self as TemplateBasedGenerator>::base_path(self)
    }

    fn generate_files(&self, project: &Project, file_set: &mut FileSet) -> Result<()> {
        let mut template = Template::new();
        self.register_templates(&mut template)?;
        register_functions!(template, name_from_path, join_path);
        self.register_functions(project, &mut template);
        self.generate_module(project, &project.root_module, file_set, &template)?;
        Ok(())
    }
}
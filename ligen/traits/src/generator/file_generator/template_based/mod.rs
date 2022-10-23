use super::{FileGenerator, FileSet};

use ligen_ir::{Module, Project};
use ligen_common::*;

use std::path::PathBuf;

pub use handlebars;

#[macro_export]
macro_rules! add_template {
    ($template:ident, $identifier:ident) => {
        // TODO: Stop using expect and use ? instead?
        $template.register_template_string(stringify!($identifier), include_str!(concat!("templates/", stringify!($identifier), ".hbs"))).expect(concat!("Failed to load ", stringify!($identifier), " template."));
    }
}

// TODO: Use #[derive(TemplateFiles)] when https://github.com/rust-lang/rust/issues/54725 is stable.
//  The idea is to iterate over the templates folder and automatically register all the template files.
#[macro_export]
macro_rules! templates {
    ($($identifier:ident),+) => {
        {
            let mut template = Handlebars::new();
            $($crate::add_template!(template, $identifier);)+
            template
        }
    }
}

pub trait TemplateSetup {
    fn get_template(&self) -> Result<handlebars::Handlebars>;
}

pub trait TemplateBasedGenerator: TemplateSetup {
    fn get_functions(&self, project: &Project, template: &mut handlebars::Handlebars);

    fn base_path(&self) -> PathBuf;

    fn generate_module(&self, project: &Project, module: &Module, file_set: &mut FileSet, template: &handlebars::Handlebars) -> Result<()>;
}

impl <T: TemplateBasedGenerator> FileGenerator for T {
    fn base_path(&self) -> PathBuf {
        <Self as TemplateBasedGenerator>::base_path(self)
    }

    fn generate_files(&self, project: &Project, file_set: &mut FileSet) -> Result<()> {
        let mut template = self.get_template()?;
        self.get_functions(project, &mut template);
        self.generate_module(project, &project.root_module, file_set, &template,)?;
        Ok(())
    }
}
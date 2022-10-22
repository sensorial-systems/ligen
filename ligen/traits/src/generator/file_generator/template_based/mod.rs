use crate::generator::{FileGenerator, FileSet};

use ligen_ir::{Module, Project};
use ligen_common::*;

use std::path::PathBuf;

pub use handlebars;

pub trait TemplateBasedGenerator {
    fn get_template(&self) -> Result<handlebars::Handlebars>;

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
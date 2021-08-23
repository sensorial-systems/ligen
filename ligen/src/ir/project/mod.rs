//! Project representation.

mod arguments;
pub use arguments::Arguments;

use crate::generator::TemporaryFFIProject;
use crate::ir::Module;
use crate::prelude::*;


/// Project representation.
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct Project {
    // TODO: Maybe the fields of Arguments should be moved to Project.
    pub arguments: Arguments,
    pub root_module: Module
}

impl Project {
    fn check_build() -> Result<()> {
        // The project isn't available if we are currently building the TemporaryProject.
        if TemporaryFFIProject::is_building() {
            Err(Error::Message("Use if let Ok(project) = Project::read() { ... }.".into()))
        } else {
            Ok(())
        }
    }

    /// Read the current project AST.
    pub fn read() -> Result<Self> {
        Self::check_build()?;
        let root_module = Module::root()?;
        Self::read_from_module(root_module)
    }

    /// Read the current project AST from the specified module.
    pub fn read_from_module(root_module: Module) -> Result<Self> {
        Self::check_build()?;
        let arguments = Arguments::from_env()?;
        Ok(Self { arguments, root_module })
    }

}

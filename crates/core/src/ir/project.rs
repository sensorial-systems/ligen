//! Project representation.

use crate::prelude::*;
use crate::ir::Module;
use crate::generator::Arguments;

/// Project representation.
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct Project {
    // TODO: Maybe the fields of Arguments should be moved to Project.
    pub arguments: Arguments,
    pub root_module: Module
}

impl Project {
    /// Read the current project AST.
    pub fn read() -> Result<Self> {
        let root_module = Module::root()?;
        let arguments = Arguments::from_env()?;
        Ok(Self { arguments, root_module })
    }
}
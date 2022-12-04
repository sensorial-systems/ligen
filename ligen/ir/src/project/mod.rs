//! Project representation.

use crate::Module;
use crate::prelude::*;
use crate::conventions::naming::NamingConvention;
use std::path::PathBuf;

// TODO: I think the only reason it exists ie because Project is Rusty. We should merge and generalize the concepts.
// FIXME: This should be removed asap.
/// Project info.
#[derive(Clone)]
pub struct ProjectInfo {
    pub directory: PathBuf,
    pub name: NamingConvention
}

/// Project representation.
#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Project {
    pub directory: PathBuf,
    pub name: NamingConvention,
    pub root_module: Module,
}

impl Project {
    /// Project directory path.
    pub fn directory(&self) -> &std::path::Path {
        self.directory.as_path()
    }
    /// Get the project name.
    pub fn name(&self) -> &NamingConvention {
        &self.name
    }
}

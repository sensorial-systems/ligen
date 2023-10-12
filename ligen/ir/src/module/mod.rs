//! Module representation.

pub mod import;

#[cfg(any(test, feature = "mocks"))]
pub mod mock;

pub use import::*;

use crate::prelude::*;
use crate::{Path, Visibility, Attributes, Function, Constant, Identifier, TypeDefinition};
use crate::interface::Interface;

/// Module representation.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Module {
    /// Attributes.
    pub attributes: Attributes,
    /// Visibility.
    pub visibility: Visibility,
    /// Module identifier
    pub identifier: Identifier,
    /// Imports.
    pub imports: Vec<Import>,
    /// Constants.
    pub constants: Vec<Constant>,
    /// Functions.
    pub functions: Vec<Function>,
    /// Types.
    pub types: Vec<TypeDefinition>,
    /// Interfaces.
    pub interfaces: Vec<Interface>,
    /// Sub-modules.
    pub modules: Vec<Module>,
}

impl Module {
    /// Find the module with the specified path.
    pub fn find_module(&self, path: &Path) -> Option<&Module> {
        let mut path = path.clone();
        if let Some(identifier) = path.pop_back() {
            if path.segments.is_empty() && self.identifier == identifier {
                Some(self)
            } else {
                self
                    .modules
                    .iter()
                    .filter_map(|module| module.find_module(&path))
                    .next()
            }
        } else {
            None
        }
    }
}

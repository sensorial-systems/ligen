//! Module representation.

pub mod import;

#[cfg(any(test, feature = "mocks"))]
pub mod mock;

pub use import::*;

use crate::prelude::*;
use crate::{Path, Visibility, Attributes, Function, Object, Identifier, TypeDefinition};
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
    /// Objects.
    pub objects: Vec<Object>,
    /// Functions.
    pub functions: Vec<Function>,
    /// Types.
    pub types: Vec<TypeDefinition>,
    /// Interfaces.
    pub interfaces: Vec<Interface>,
    /// Sub-modules.
    pub modules: Vec<Module>,
}

impl CountSymbols for Module {
    fn count_symbols(&self) -> usize {
        self.objects.len()
            + self.functions.count_symbols()
            + self.types.count_symbols()
            + self.interfaces.count_symbols()
            + self.modules.count_symbols()
    }
}

impl CountSymbols for &mut Module {
    fn count_symbols(&self) -> usize {
        self.objects.len()
            + self.functions.count_symbols()
            + self.types.count_symbols()
            + self.interfaces.count_symbols()
            + self.modules.count_symbols()
    }    
}

impl CountSymbols for &Module {
    fn count_symbols(&self) -> usize {
        self.objects.len()
            + self.functions.count_symbols()
            + self.types.count_symbols()
            + self.interfaces.count_symbols()
            + self.modules.count_symbols()
    }    
}

impl CountSymbols for Vec<Module> {
    fn count_symbols(&self) -> usize {
        self.iter().fold(0, |acc, module| acc + module.count_symbols())
    }
}

impl CountSymbols for &Vec<Module> {
    fn count_symbols(&self) -> usize {
        self.iter().fold(0, |acc, module| acc + module.count_symbols())
    }
}

impl Module {
    /// Find the module with the specified path.
    pub fn find_module(&self, path: &Path) -> Option<&Module> {
        let mut path = path.clone();
        if let Some(path_segment) = path.pop_back() {
            if path.segments.is_empty() && self.identifier == path_segment.identifier {
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

    pub fn join(&mut self, other: Self) {
        self.interfaces.extend(other.interfaces);
        self.functions.extend(other.functions);
        self.types.extend(other.types);
        self.objects.extend(other.objects);
        self.modules.extend(other.modules);
        self.imports.extend(other.imports);
    }

    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
            && self.functions.is_empty()
            && self.interfaces.is_empty()
            && self.types.is_empty()
            && self.modules.is_empty()
    }
}

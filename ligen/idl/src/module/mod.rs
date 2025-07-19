//! Module representation.

pub mod import;

#[cfg(any(test, feature = "mocks"))]
pub mod mock;

pub use import::*;
use ::is_tree::*;

use crate::prelude::*;
use crate::{Visibility, Attributes, Function, Object, Identifier, TypeDefinition, Interface};

/// Module representation.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Module<Block = ()> {
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
    pub functions: Vec<Function<Block>>,
    /// Types.
    pub types: Vec<TypeDefinition>,
    /// Interfaces.
    pub interfaces: Vec<Interface>,
    /// Sub-modules.
    pub modules: Vec<Module<Block>>,
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

impl<'a, Block> HasBranches<&'a Module<Block>> for &'a Module<Block> {
    fn branches_impl(self) -> impl Iterator<Item = &'a Module<Block>> {
        self.modules.iter()
    }
}

impl<'a, Block> HasBranches<&'a mut Module<Block>> for &'a mut Module<Block> {
    fn branches_impl(self) -> impl Iterator<Item = &'a mut Module<Block>> {
        self.modules.iter_mut()
    }
}

impl<Block> HasPathSegment for &Module<Block> {
    fn path_segment(&self) -> String {
        self.identifier.to_string()
    }
}

impl<Block> HasPathSegment for &mut Module<Block> {
    fn path_segment(&self) -> String {
        self.identifier.to_string()
    }
}

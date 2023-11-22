//! Module representation.

pub mod import;

#[cfg(any(test, feature = "mocks"))]
pub mod mock;

pub use import::*;
use is_tree::{IsTree, HasIdentifier, IntoIterTypeMut, TypeIteratorMut};

use crate::{prelude::*, Type};
use crate::{Visibility, Attributes, Function, Object, Identifier, TypeDefinition};
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

impl HasIdentifier for Module {
    type Identifier = Identifier;
    fn identifier(&self) -> &Self::Identifier {
        &self.identifier
    }
}

impl IsTree for Module {
    fn branches<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Self> + 'a> {
        Box::new(self.modules.iter())
    }
    
    fn branches_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut Self> + 'a> {
        Box::new(self.modules.iter_mut())
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

impl IntoIterTypeMut<Type> for Module {
    fn into_type_iterator<'a>(&'a mut self) -> TypeIteratorMut<'a, Type> {
        let mut stack = Vec::new();
        stack.extend(self.interfaces.iter_mut().flat_map(|i| i.into_type_iterator()));
        stack.extend(self.functions.iter_mut().flat_map(|f| f.into_type_iterator()));
        stack.extend(self.types.iter_mut().flat_map(|t| t.into_type_iterator()));
        stack.extend(self.objects.iter_mut().flat_map(|o| o.into_type_iterator()));
        stack.extend(self.modules.iter_mut().flat_map(|m| m.into_type_iterator()));
        stack.into()
    }
}
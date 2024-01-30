//! Module representation.

pub mod import;

#[cfg(any(test, feature = "mocks"))]
pub mod mock;

pub use import::*;
use ::is_tree::*;

use crate::{prelude::*, Type, Method};
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

impl KnowsPathSegment for Module {
    type PathSegment = Identifier;
}

impl HasPathSegment for Module {
    fn path_segment(&self) -> &Self::PathSegment {
        &self.identifier
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

// FIXME: Remove this.
// impl IntoIterTypeMut<Type> for Module {
//     fn type_iterator(&mut self) -> TypeIterMut<'_, Type> {
//         let mut stack = Vec::new();
//         stack.extend(self.interfaces.iter_mut().flat_map(|i| i.type_iterator()));
//         stack.extend(self.functions.iter_mut().flat_map(|f| f.type_iterator()));
//         stack.extend(self.types.iter_mut().flat_map(|t| t.type_iterator()));
//         stack.extend(self.objects.iter_mut().flat_map(|o| o.type_iterator()));
//         stack.extend(self.modules.iter_mut().flat_map(|m| m.type_iterator()));
//         stack.into()
//     }
// }

// impl IntoIterTypeMut<Method> for Module {
//     fn type_iterator(&mut self) -> TypeIterMut<'_, Method> {
//         let mut stack = Vec::new();
//         stack.extend(self.interfaces.iter_mut().flat_map(|i| i.iter_type_mut::<Method>()));
//         stack.extend(self.modules.iter_mut().flat_map(|m| m.iter_type_mut::<Method>()));
//         stack.into()
//     }
// }

// impl IntoIterTypeMut<Interface> for Module {
//     fn type_iterator(&mut self) -> TypeIterMut<'_, Interface> {
//         let mut stack = self.interfaces.iter_mut().collect::<Vec<_>>();
//         stack.extend(self.modules.iter_mut().flat_map(|m| m.iter_type_mut::<Interface>()));
//         stack.into()
//     }
// }
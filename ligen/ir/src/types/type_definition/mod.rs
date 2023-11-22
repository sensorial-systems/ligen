//! Type definitions.

pub mod kind_definition;

use is_tree::{IntoIterTypeMut, TypeIteratorMut};
pub use kind_definition::*;

use crate::{prelude::*, Attributes, Visibility, Path, Generics, Type};
use crate::Identifier;

/// All the possible ways to define a type.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeDefinition {
    /// Definition attributes.
    pub attributes: Attributes,
    /// Definition visibility.
    pub visibility: Visibility,
    /// Definition identifier.
    pub identifier: Identifier,
    /// Generic parameters.
    pub generics: Generics,
    /// Interfaces that this definition implements.
    pub interfaces: Vec<Path>,
    /// Specific definition of the kind (e.g. Structure, Enumeration).
    pub definition: KindDefinition
}

impl CountSymbols for Vec<TypeDefinition> {
    fn count_symbols(&self) -> usize {
        self.len()
    }
}

impl CountSymbols for &Vec<TypeDefinition> {
    fn count_symbols(&self) -> usize {
        self.len()
    }
}

impl IntoIterTypeMut<Type> for TypeDefinition {
    fn into_type_iterator<'a>(&'a mut self) -> TypeIteratorMut<'a, Type> {
        let mut stack = Vec::new();
        stack.extend(self.interfaces.iter_mut().flat_map(|m| m.into_type_iterator()));
        stack.extend(self.definition.into_type_iterator());
        stack.extend(self.generics.into_type_iterator());
        stack.into()
    }
}

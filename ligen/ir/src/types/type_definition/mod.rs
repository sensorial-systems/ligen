//! Type definitions.

pub mod kind_definition;

use is_tree::{IntoIterTypeMut, TypeIterMut};
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
    fn type_iterator(&mut self) -> TypeIterMut<'_, Type> {
        let mut stack = Vec::new();
        stack.extend(self.interfaces.iter_mut().flat_map(|m| m.type_iterator()));
        stack.extend(self.definition.type_iterator());
        stack.extend(self.generics.type_iterator());
        stack.into()
    }
}

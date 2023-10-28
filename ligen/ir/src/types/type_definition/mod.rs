//! Type definitions.

pub mod kind_definition;

pub use kind_definition::*;

use crate::{prelude::*, Attributes, Visibility, Path};
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

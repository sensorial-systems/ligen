//! Type definitions.

mod structure;
mod enumeration;

pub use structure::*;
pub use enumeration::*;
use crate::ir::{Identifier, Visibility};

// TODO: Bring common properties from Structure and Enumeration to TypeDefinition.
/// All the possible ways to define a type.
#[derive(Debug, Clone, PartialEq)]
#[allow(missing_docs)]
pub enum TypeDefinition {
    Structure(Structure),
    Enumeration(Enumeration)
}

impl TypeDefinition {
    /// Get the type definition identifier.
    pub fn identifier(&self) -> &Identifier {
        match self {
            Self::Structure(structure) => &structure.identifier,
            Self::Enumeration(enumeration) => &enumeration.identifier
        }
    }

    /// Get the type definition visibility.
    pub fn visibility(&self) -> &Visibility {
        match self {
            Self::Structure(structure) => &structure.visibility,
            Self::Enumeration(enumeration) => &enumeration.visibility
        }
    }
}

impl From<Structure> for TypeDefinition {
    fn from(structure: Structure) -> Self {
        Self::Structure(structure)
    }
}

impl From<Enumeration> for TypeDefinition {
    fn from(enumeration: Enumeration) -> Self {
        Self::Enumeration(enumeration)
    }
}
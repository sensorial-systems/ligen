//! Type definitions.

mod structure;
mod enumeration;

pub use structure::*;
pub use enumeration::*;

/// All the possible ways to define a type.
#[derive(Debug, Clone, PartialEq)]
#[allow(missing_docs)]
pub enum TypeDefinition {
    Structure(Structure),
    Enumeration(Enumeration)
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
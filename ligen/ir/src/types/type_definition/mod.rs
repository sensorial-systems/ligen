//! Type definitions.

pub mod structure;
pub mod enumeration;

use crate::prelude::*;
pub use structure::*;
pub use enumeration::*;

/// All the possible ways to define a type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum TypeDefinition {
    Structure(Structure),
    Enumeration(Enumeration)
}

impl Default for TypeDefinition {
    fn default() -> Self {
        Self::Structure(Default::default())
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
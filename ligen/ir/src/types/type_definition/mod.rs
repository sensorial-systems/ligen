//! Type definitions.

pub mod structure;
pub mod enumeration;

use crate::prelude::*;
pub use structure::{Structure, Field};
pub use enumeration::{Enumeration, Variant};
use crate::Identifier;

/// All the possible ways to define a type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum TypeDefinition {
    Structure(Structure),
    Enumeration(Enumeration)
}

impl TypeDefinition {
    pub fn identifier(&self) -> &Identifier {
        match self {
            Self::Structure(structure) => &structure.identifier,
            Self::Enumeration(enumeration) => &enumeration.identifier
        }
    }

    pub fn identifier_mut(&mut self) -> &mut Identifier {
        match self {
            Self::Structure(structure) => &mut structure.identifier,
            Self::Enumeration(enumeration) => &mut enumeration.identifier
        }
    }
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
//! Type definitions.

mod structure;
mod enumeration;

use crate::prelude::*;
pub use structure::*;
pub use enumeration::*;
use crate::{Path, Visibility};

// TODO: Bring common properties from Structure and Enumeration to TypeDefinition.
/// All the possible ways to define a type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum TypeDefinition {
    Structure(Structure),
    Enumeration(Enumeration)
}

impl TypeDefinition {
    /// Get the path definition identifier.
    pub fn path(&self) -> &Path {
        match self {
            Self::Structure(structure) => &structure.path,
            Self::Enumeration(enumeration) => &enumeration.path
        }
    }

    pub fn path_mut(&mut self) -> &mut Path {
        match self {
            Self::Structure(structure) => &mut structure.path,
            Self::Enumeration(enumeration) => &mut enumeration.path
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
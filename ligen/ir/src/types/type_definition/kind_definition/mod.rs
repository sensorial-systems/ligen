pub mod structure;
pub mod enumeration;
pub mod type_alias;

use crate::prelude::*;

pub use structure::{Structure, Field};
pub use enumeration::{Enumeration, Variant};
pub use type_alias::TypeAlias;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum KindDefinition {
    Structure(Structure),
    Enumeration(Enumeration),
    TypeAlias(TypeAlias)
}

impl KindDefinition {
    /// Returns the name of the kind.
    pub fn kind_name(&self) -> &'static str {
        match self {
            Self::Structure(_) => "Structure",
            Self::Enumeration(_) => "Enumeration",
            Self::TypeAlias(_) => "TypeAlias"
        }
    }

    /// Returns `true` if the kind is empty.
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Structure(structure) => structure.fields.is_empty(),
            Self::Enumeration(enumeration) => enumeration.variants.is_empty(),
            Self::TypeAlias(_) => false
        }
    }

    /// Returns the number of items in the kind.
    pub fn count(&self) -> usize {
        match self {
            Self::Structure(structure) => structure.fields.len(),
            Self::Enumeration(enumeration) => enumeration.variants.len(),
            Self::TypeAlias(_) => 0
        }
    
    }
}

impl Default for KindDefinition {
    fn default() -> Self {
        Self::Structure(Default::default())
    }
}

impl From<Structure> for KindDefinition {
    fn from(structure: Structure) -> Self {
        Self::Structure(structure)
    }
}

impl From<Enumeration> for KindDefinition {
    fn from(enumeration: Enumeration) -> Self {
        Self::Enumeration(enumeration)
    }
}

impl From<TypeAlias> for KindDefinition {
    fn from(value: TypeAlias) -> Self {
        Self::TypeAlias(value)
    }
}
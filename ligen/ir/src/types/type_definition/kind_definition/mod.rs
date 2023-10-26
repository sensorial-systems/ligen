pub mod structure;
pub mod enumeration;

use crate::prelude::*;

pub use structure::{Structure, Field};
pub use enumeration::{Enumeration, Variant};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum KindDefinition {
    Structure(Structure),
    Enumeration(Enumeration)
}

impl KindDefinition {
    /// Returns the name of the kind.
    pub fn kind_name(&self) -> &'static str {
        match self {
            Self::Structure(_) => "Structure",
            Self::Enumeration(_) => "Enumeration"
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
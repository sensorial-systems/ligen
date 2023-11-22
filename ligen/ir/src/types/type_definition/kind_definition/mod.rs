pub mod structure;
pub mod enumeration;

use crate::{prelude::*, Type};

use is_tree::{IntoIterTypeMut, TypeIteratorMut};
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

    /// Returns `true` if the kind is empty.
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Structure(structure) => structure.fields.is_empty(),
            Self::Enumeration(enumeration) => enumeration.variants.is_empty()
        }
    }

    /// Returns the number of items in the kind.
    pub fn count(&self) -> usize {
        match self {
            Self::Structure(structure) => structure.fields.len(),
            Self::Enumeration(enumeration) => enumeration.variants.len()
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

impl IntoIterTypeMut<Type> for KindDefinition {
    fn into_type_iterator<'a>(&'a mut self) -> TypeIteratorMut<'a, Type> {
        match self {
            Self::Structure(structure) => structure.into_type_iterator(),
            Self::Enumeration(enumeration) => enumeration.into_type_iterator()
        }
    }
}
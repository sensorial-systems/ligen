use crate::{Mutability, Type};
use crate::prelude::*;

/// Reference representation.
#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Reference {
    /// Mutability.
    pub mutability: Mutability,
    /// The type being referenced.
    pub type_: Box<Type>
}

impl std::fmt::Display for Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.mutability {
            Mutability::Constant => f.write_str("*const ")?,
            Mutability::Mutable => f.write_str("*mut ")?
        }
        f.write_str(&self.type_.to_string())
    }
}

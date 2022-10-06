//! Function parameter.

use crate::prelude::*;
use crate::{Identifier, Type, Attributes, Mutability};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
/// Parameter representation.
pub struct Parameter {
    /// Attributes.
    pub attributes: Attributes,
    /// Identifier.
    pub identifier: Identifier,
    /// Type.
    pub type_: Type,
}

impl Parameter {
    /// Get parameter mutability.
    pub fn mutability(&self) -> Mutability {
        match &self.type_ {
            Type::Reference(reference) => reference.mutability,
            _ => Mutability::Constant
        }
    }
}

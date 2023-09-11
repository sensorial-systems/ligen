//! Attribute enumeration.

use crate::prelude::*;
use crate::{Literal, Identifier, Attributes};

/// Attribute enueration.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Attribute {
    /// Literal Variant
    Literal(Literal),
    /// Named Variant
    Named(Identifier, Literal),
    /// Group Variant
    Group(Identifier, Attributes),
}

impl Default for Attribute {
    fn default() -> Self {
        Self::Literal(Literal::default())
    }
}
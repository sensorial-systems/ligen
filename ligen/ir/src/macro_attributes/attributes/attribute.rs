//! Attribute enumeration.

use std::fmt::{Display, Formatter};
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

impl Display for Attribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Attribute::Literal(literal) => write!(f, "{}", literal),
            Attribute::Named(identifier, literal) => write!(f, "{} = {}", identifier, literal),
            Attribute::Group(identifier, attributes) => write!(f, "{}({})", identifier, attributes),
        }
    }
}
//! Attribute enumeration.

pub mod named;
pub mod group;

#[cfg(any(test, feature = "mocks"))]
pub mod mock;

use std::fmt::{Display, Formatter};
use crate::prelude::*;
use crate::Literal;

pub use group::*;

pub use named::*;

/// Attribute enueration.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Attribute {
    /// Literal Variant
    Literal(Literal),
    /// Named Variant
    Named(Named),
    /// Group Variant
    Group(Group),
}

impl From<Literal> for Attribute {
    fn from(literal: Literal) -> Self {
        Self::Literal(literal)
    }
}

impl From<Named> for Attribute {
    fn from(named: Named) -> Self {
        Self::Named(named)
    }
}

impl From<Group> for Attribute {
    fn from(group: Group) -> Self {
        Self::Group(group)
    }
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
            Attribute::Named(named) => write!(f, "{}", named),
            Attribute::Group(group) => write!(f, "{}", group),
        }
    }
}
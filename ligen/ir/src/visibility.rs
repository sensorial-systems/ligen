//! Visibility enumeration.

use std::fmt::Display;

use crate::prelude::*;

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, EnumIter)]
/// Visibility enumeration.
pub enum Visibility {
    /// Private
    Private,
    /// Public
    Public,
}

impl Default for Visibility {
    fn default() -> Self {
        Self::Public
    }
}

impl Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

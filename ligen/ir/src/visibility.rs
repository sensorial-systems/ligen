//! Visibility enumeration.

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

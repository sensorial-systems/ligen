//! Visibility enumeration.

use std::fmt::Display;

use crate::prelude::*;

#[derive(
    Debug, PartialEq, Clone, Copy, Serialize, Deserialize, EnumIter, JsonSchema, EnumAsInner,
)]
/// Visibility enumeration.
#[derive(Default)]
pub enum Visibility {
    /// Private
    Private,
    /// Public
    #[default]
    Public,
}


impl Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

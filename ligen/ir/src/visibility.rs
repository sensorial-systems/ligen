//! Visibility enumeration.

use crate::prelude::*;

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
/// Visibility enumeration.
pub enum Visibility {
    /// Public
    Public,
    /// Crate
    Crate,
    /// Restricted
    Restricted,
    /// Inherited
    Inherited,
}

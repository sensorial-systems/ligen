//! Visibility enumeration.

use crate::prelude::*;

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
/// Visibility enumeration.
pub enum Visibility {
    /// Public
    Public,
    // FIXME: This is rusty.
    /// Crate
    Crate,
    // FIXME: This is rusty.
    /// Restricted
    Restricted,
    // FIXME: This is rusty.
    /// Inherited
    Inherited,
}

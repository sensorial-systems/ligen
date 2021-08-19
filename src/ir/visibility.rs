//! Visibility enumeration.

#[derive(Debug, PartialEq, Clone, Copy)]
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

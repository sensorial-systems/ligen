//! Object representation with its structure and implementation items.

use crate::ir::{Implementation, Structure, Path};

/// Object representation.
/// This is a conventional structure created from a composition of different associated type elements.
#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    /// Object path.
    pub path: Path,
    /// Object structure. If it's None, then the object is an opaque type.
    pub structure: Option<Structure>,
    /// Object implementations blocks.
    pub implementations: Vec<Implementation>
}

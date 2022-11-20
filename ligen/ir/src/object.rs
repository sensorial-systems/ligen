//! `Object` can be defined as a `Structure` and an `Enumeration`.

use crate::prelude::*;
use crate::{Implementation, TypeDefinition};

/// Object representation.
/// This is a conventional structure created from a composition of a struct definition and impl
/// blocks.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Object {
    /// Object type definition.
    pub definition: TypeDefinition,
    /// Object implementations blocks.
    pub implementations: Vec<Implementation> // FIXME: Rusty.
}

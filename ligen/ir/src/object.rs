//! `Object` can be defined as a `Structure` and an `Enumeration`.

use crate::prelude::*;
use crate::{Constant, Implementation, TypeDefinition};

/// Object representation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Object {
    // FIXME: Rename to ObjectData? Move Constant to ObjectData?
    /// Object type definition.
    pub definition: TypeDefinition,
    // FIXME: Rename to ObjectFunction?
    /// Object implementations blocks.
    pub implementation: Implementation
}

// TODO: Use this instead.
// pub struct Object {
//     pub definition: TypeDefinition,
//     pub constants: Vec<Constant>,
//     pub methods: Vec<Method>
// }
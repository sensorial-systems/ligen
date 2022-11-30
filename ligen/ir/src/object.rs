//! `Object` can be defined as a `Structure` and an `Enumeration`.

use crate::prelude::*;
use crate::{Constant, Function, TypeDefinition, method::Method};

/// Object representation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Object {
    /// Object type definition.
    pub definition: TypeDefinition,
    /// Object associated constants.
    pub constants: Vec<Constant>,
    /// Object associated functions.
    pub functions: Vec<Function>,
    /// Object methods.
    pub methods: Vec<Method>
}

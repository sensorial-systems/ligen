//! `Object` can be defined as a `Structure` and an `Enumeration`.

use crate::prelude::*;
use crate::{Constant, Function, TypeDefinition, method::Method, Structure, Enumeration};

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

impl From<TypeDefinition> for Object {
    fn from(definition: TypeDefinition) -> Self {
        Self {
            definition,
            constants: Default::default(),
            functions: Default::default(),
            methods: Default::default()
        }
    }
}

impl From<Structure> for Object {
    fn from(structure: Structure) -> Self {
        TypeDefinition::Structure(structure).into()
    }
}

impl From<Enumeration> for Object {
    fn from(enumeration: Enumeration) -> Self {
        TypeDefinition::Enumeration(enumeration).into()
    }
}
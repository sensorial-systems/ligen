//! `Object` can be defined as a `Structure` and an `Enumeration`.

use crate::prelude::*;
use crate::{Constant, Function, TypeDefinition, Method, Attributes, Visibility, Identifier};

/// Object representation.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Object {
    /// Attributes field.
    pub attributes: Attributes,
    /// Object's visibility.
    pub visibility: Visibility,
    /// Object's identifier.
    pub identifier: Identifier,
    /// Object type definition.
    pub definition: TypeDefinition,
    /// Object associated constants.
    pub constants: Vec<Constant>,
    /// Object associated functions.
    pub functions: Vec<Function>,
    /// Object methods.
    pub methods: Vec<Method>
}

// FIXME: Does it still make sense?
impl From<TypeDefinition> for Object {
    fn from(definition: TypeDefinition) -> Self {
        Self {
            definition,
            attributes: Default::default(),
            visibility: Default::default(),
            identifier: Default::default(),
            constants: Default::default(),
            functions: Default::default(),
            methods: Default::default()
        }
    }
}

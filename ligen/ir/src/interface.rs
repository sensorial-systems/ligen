use crate::{Attributes, Constant, Function, Identifier, Method, Path, Visibility};
use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Interface {
    /// Interface attributes.
    pub attributes: Attributes,
    /// Interface visibility.
    pub visibility: Visibility,
    /// Interface identifier.
    pub identifier: Identifier,
    /// Public constants.
    pub constants: Vec<Constant>,
    /// Public functions.
    pub functions: Vec<Function>,
    /// Interface methods.
    pub methods: Vec<Method>,
    /// Interfaces that this interface extends.
    pub interfaces: Vec<Path>
}

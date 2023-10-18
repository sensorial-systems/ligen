use crate::{Attributes, Constant, Function, Identifier, Method, Path, Visibility};
use crate::prelude::*;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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

impl Interface {
    /// Count the number of symbols in this interface.
    pub fn count_symbols(&self) -> usize {
        self.constants.len()
        + self.functions.len()
        + self.methods.len()
    }
}
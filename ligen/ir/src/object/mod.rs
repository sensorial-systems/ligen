use crate::{Identifier, Literal, Type, Mutability};
use crate::prelude::*;

#[cfg(any(test, feature = "mocks"))]
pub mod mock;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
/// Object struct
pub struct Object {
    /// Object's mutability.
    pub mutability: Mutability,
    /// Object's identifier.
    pub identifier: Identifier,
    /// Object's type.
    pub type_: Type,
    /// Object's literal value.
    pub literal: Literal,
}

impl CountSymbols for Vec<Object> {
    fn count_symbols(&self) -> usize {
        self.len()
    }
}

impl CountSymbols for &Vec<Object> {
    fn count_symbols(&self) -> usize {
        self.len()
    }
}

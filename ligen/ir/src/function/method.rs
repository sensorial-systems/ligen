use crate::prelude::*;
use crate::{Synchrony, Attributes, Mutability, Parameter, Type, Visibility, Identifier, Block};

/// Method structure.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Method {
    /// Attributes field.
    pub attributes: Attributes,
    /// The owner mutability.
    pub mutability: Mutability,
    /// Visibility field.
    pub visibility: Visibility,
    /// Synchrony field.
    pub synchrony: Synchrony,
    /// Method's identifier.
    pub identifier: Identifier,
    /// Inputs field.
    pub inputs: Vec<Parameter>,
    /// Output field.
    pub output: Option<Type>,
    /// Body field.
    pub body: Option<Block>
}

impl CountSymbols for &Vec<Method> {
    fn count_symbols(&self) -> usize {
        self.len()
    }
}

impl CountSymbols for Vec<Method> {
    fn count_symbols(&self) -> usize {
        self.len()
    }
}

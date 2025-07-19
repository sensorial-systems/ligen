use crate::prelude::*;
use crate::{Synchrony, Attributes, Mutability, Parameter, Type, Visibility, Identifier};

/// Method structure.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Method<Body = ()> {
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
    pub body: Body
}

impl<Body> CountSymbols for &Vec<Method<Body>> {
    fn count_symbols(&self) -> usize {
        self.len()
    }
}

impl<Body> CountSymbols for Vec<Method<Body>> {
    fn count_symbols(&self) -> usize {
        self.len()
    }
}

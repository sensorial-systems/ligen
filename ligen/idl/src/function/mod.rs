use crate::prelude::*;

use crate::{Attributes, Identifier, Type, Visibility};

pub mod parameter;
pub mod method;
pub mod synchrony;

pub use parameter::*;
pub use method::*;
pub use synchrony::*;

/// Function structure.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Function {
    /// Attributes field.
    pub attributes: Attributes,
    /// Visibility field.
    pub visibility: Visibility,
    /// Synchrony field.
    pub synchrony: Synchrony,
    /// Function's identifier.
    pub identifier: Identifier,
    /// Inputs field.
    pub inputs: Vec<Parameter>,
    /// Output field.
    pub output: Option<Type>,
    // TODO: What to do about the body?
    // /// Body field.
    // pub body: Option<Block>
}

impl Function {
    pub fn new<R: Into<Type>>(identifier: impl Into<Identifier>, inputs: impl IntoIterator<Item = Parameter>, output: Option<R>) -> Self {
    // TODO: Add body
    // pub fn new<R: Into<Type>, B: Into<Block>>(identifier: impl Into<Identifier>, inputs: impl IntoIterator<Item = Parameter>, output: Option<R>, body: Option<B>) -> Self {
        Self {
            attributes: Attributes::default(),
            visibility: Visibility::Public,
            synchrony: Synchrony::Synchronous,
            identifier: identifier.into(),
            inputs: inputs.into_iter().collect(),
            output: output.map(Into::into),
        }
    }
}

impl CountSymbols for Vec<Function> {
    fn count_symbols(&self) -> usize {
        self.len()
    }
}

impl CountSymbols for &Vec<Function> {
    fn count_symbols(&self) -> usize {
        self.len()
    }
}

impl From<Method> for Function {
    fn from(method: Method) -> Self {
        Self {
            attributes: method.attributes,
            visibility: method.visibility,
            synchrony: method.synchrony,
            identifier: method.identifier,
            inputs: method.inputs,
            output: method.output,
            // TODO: Add body
            // body: method.body,
        }
    }
}

#[cfg(any(test, feature = "mocks"))]
pub mod mock;
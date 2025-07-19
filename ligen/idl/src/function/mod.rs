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
pub struct Function<Body = ()> {
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
    /// Body field.
    pub body: Body
}

impl<Body> Function<Body> {
    pub fn new<R: Into<Type>, B: Into<Body>>(identifier: impl Into<Identifier>, inputs: impl IntoIterator<Item = Parameter>, output: Option<R>, body: B) -> Self {
        Self {
            attributes: Attributes::default(),
            visibility: Visibility::Public,
            synchrony: Synchrony::Synchronous,
            identifier: identifier.into(),
            inputs: inputs.into_iter().collect(),
            output: output.map(Into::into),
            body: body.into(),
        }
    }
}

impl<Body> CountSymbols for Vec<Function<Body>> {
    fn count_symbols(&self) -> usize {
        self.len()
    }
}

impl<Body> CountSymbols for &Vec<Function<Body>> {
    fn count_symbols(&self) -> usize {
        self.len()
    }
}

impl<Body> From<Method<Body>> for Function<Body> {
    fn from(method: Method<Body>) -> Self {
        Self {
            attributes: method.attributes,
            visibility: method.visibility,
            synchrony: method.synchrony,
            identifier: method.identifier,
            inputs: method.inputs,
            output: method.output,
            body: method.body,
        }
    }
}

#[cfg(any(test, feature = "mocks"))]
pub mod mock;
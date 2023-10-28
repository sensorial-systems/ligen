use crate::prelude::*;

use crate::{Attributes, Identifier, Type, Visibility};

pub mod parameter;
pub mod method;
pub mod synchrony;

pub use parameter::*;
pub use method::*;
pub use synchrony::*;

/// Function structure.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
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

#[cfg(any(test, feature = "mocks"))]
pub mod mock;
use crate::prelude::*;

use crate::{Attributes, Identifier, Type, Visibility};

pub mod parameter;
pub mod method;
pub mod synchrony;

use is_tree::{IntoIterTypeMut, TypeIteratorMut};
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

impl IntoIterTypeMut<Type> for Function {
    fn into_type_iterator<'a>(&'a mut self) -> TypeIteratorMut<'a, Type> {
        let mut stack = Vec::new();
        stack.extend(self.inputs.iter_mut().flat_map(|m| m.into_type_iterator()));
        stack.extend(self.output.iter_mut().flat_map(|m| m.into_type_iterator()));
        stack.into()
    }
}

#[cfg(any(test, feature = "mocks"))]
pub mod mock;
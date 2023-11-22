use is_tree::{IntoIterTypeMut, TypeIteratorMut};

use crate::prelude::*;
use crate::{Synchrony, Attributes, Mutability, Parameter, Type, Visibility, Identifier};

/// Method structure.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
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

impl IntoIterTypeMut<Type> for Method {
    fn into_type_iterator<'a>(&'a mut self) -> TypeIteratorMut<'a, Type> {
        let mut stack = Vec::new();
        stack.extend(self.inputs.iter_mut().flat_map(|m| m.into_type_iterator()));
        stack.extend(self.output.iter_mut().flat_map(|m| m.into_type_iterator()));
        stack.into()
    }
}

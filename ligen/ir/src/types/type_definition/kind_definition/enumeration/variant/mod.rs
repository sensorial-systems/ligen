//! Enumeration variant representation.

use is_tree::{IntoIterTypeMut, TypeIteratorMut};

use crate::{prelude::*, Type};
use crate::{Attributes, Identifier};

/// Enumeration representation.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Variant {
    /// Attributes field.
    pub attributes: Attributes,
    /// Variant identifier.
    pub identifier: Identifier
}

impl IntoIterTypeMut<Type> for Variant {
    fn into_type_iterator<'a>(&'a mut self) -> TypeIteratorMut<'a, Type> {
        vec![].into()
    }
}
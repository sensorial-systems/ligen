//! Enumeration variant representation.

use is_tree::{IntoIterTypeMut, TypeIterMut};

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
    fn type_iterator(&mut self) -> TypeIterMut<'_, Type> {
        vec![].into()
    }
}
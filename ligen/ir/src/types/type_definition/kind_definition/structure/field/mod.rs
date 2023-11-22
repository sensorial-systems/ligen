//! Structure field representation.

use is_tree::{IntoIterTypeMut, TypeIteratorMut};

use crate::prelude::*;
use crate::{Identifier, Type, Visibility, Attributes};

/// Property representation.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field {
    /// Field attributes.
    pub attributes: Attributes,
    /// Field visibility.
    pub visibility: Visibility,
    /// Field identifier.
    pub identifier: Option<Identifier>,
    /// Field type.
    pub type_: Type
}

impl IntoIterTypeMut<Type> for Field {
    fn into_type_iterator<'a>(&'a mut self) -> TypeIteratorMut<'a, Type> {
        self.type_.into_type_iterator()
    }
}
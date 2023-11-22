//! Structure representation.

#[cfg(any(test, feature = "mocks"))]
pub mod mock;

pub mod field;
pub use field::*;
use is_tree::{IntoIterTypeMut, TypeIteratorMut};

use crate::{prelude::*, Type};

/// Structure representation.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Structure {
    /// Items field.
    pub fields: Vec<Field>,
}

impl IntoIterTypeMut<Type> for Structure {
    fn into_type_iterator<'a>(&'a mut self) -> TypeIteratorMut<'a, Type> {
        self.fields.iter_mut().flat_map(|f| f.into_type_iterator()).collect::<Vec<_>>().into()
    }
}
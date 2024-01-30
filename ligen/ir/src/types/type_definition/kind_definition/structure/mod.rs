//! Structure representation.

#[cfg(any(test, feature = "mocks"))]
pub mod mock;

pub mod field;
pub use field::*;

use crate::{prelude::*, Type};

/// Structure representation.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Structure {
    /// Items field.
    pub fields: Vec<Field>,
}

// FIXME: Remove this.
// impl IntoIterTypeMut<Type> for Structure {
//     fn type_iterator(&mut self) -> TypeIterMut<'_, Type> {
//         self.fields.iter_mut().flat_map(|f| f.type_iterator()).collect::<Vec<_>>().into()
//     }
// }
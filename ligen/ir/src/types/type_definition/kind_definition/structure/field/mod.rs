//! Structure field representation.

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

// FIXME: Remove this
// impl IntoIterTypeMut<Type> for Field {
//     fn type_iterator(&mut self) -> TypeIterMut<'_, Type> {
//         self.type_.type_iterator()
//     }
// }
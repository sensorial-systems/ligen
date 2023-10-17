//! Structure representation.

#[cfg(any(test, feature = "mocks"))]
pub mod mock;

pub mod field;
pub use field::*;

use crate::prelude::*;
use crate::{Attributes, Identifier, Path, Visibility};

/// Structure representation.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Structure {
    /// Structure attributes.
    pub attributes: Attributes,
    /// Structure visibility.
    pub visibility: Visibility,
    /// Structure identifier.
    pub identifier: Identifier,
    /// Items field.
    pub fields: Vec<Field>,
    /// Interfaces that this structure implements.
    pub interfaces: Vec<Path>
}
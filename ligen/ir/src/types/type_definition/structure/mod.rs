//! Structure representation.

mod field;
pub use field::*;

use crate::prelude::*;
use crate::{Attributes, Visibility, Identifier};

/// Structure representation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Structure {
    /// Attributes field.
    pub attributes: Attributes,
    /// Structure visibility.
    pub visibility: Visibility,
    /// Structure identifier.
    pub identifier: Identifier,
    /// Items field.
    pub fields: Vec<Field>,
}

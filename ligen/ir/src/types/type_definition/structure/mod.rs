//! Structure representation.

mod field;
pub use field::*;

use crate::prelude::*;

/// Structure representation.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Structure {
    /// Items field.
    pub fields: Vec<Field>,
}
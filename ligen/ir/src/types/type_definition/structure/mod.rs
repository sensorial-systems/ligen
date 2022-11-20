//! Structure representation.

mod field;
pub use field::*;

use crate::prelude::*;
use crate::{Attributes, Visibility, Path};

/// Structure representation.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Structure {
    /// Attributes field.
    pub attributes: Attributes,
    /// Structure visibility.
    pub visibility: Visibility,
    /// Structure path.
    pub path: Path,
    /// Items field.
    pub fields: Vec<Field>,
}
//! Enumeration representation.

mod variant;
pub use variant::*;

use crate::prelude::*;
use crate::{Attributes, Visibility, Path};

/// Enumeration representation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enumeration {
    /// Attributes field.
    pub attributes: Attributes,
    /// Enumeration visibility.
    pub visibility: Visibility,
    /// Enumeration path.
    pub path: Path,
    /// Variants field.
    pub variants: Vec<Variant>,
}

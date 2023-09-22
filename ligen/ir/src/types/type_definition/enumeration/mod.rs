//! Enumeration representation.

pub mod variant;
pub use variant::*;

use crate::prelude::*;

/// Enumeration representation.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enumeration {
    /// Variants field.
    pub variants: Vec<Variant>,
}

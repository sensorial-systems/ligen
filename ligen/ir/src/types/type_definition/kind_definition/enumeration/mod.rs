//! Enumeration representation.

#[cfg(any(test, feature = "mocks"))]
pub mod mock;

pub mod variant;
pub use variant::*;

use crate::prelude::*;

/// Enumeration representation.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Enumeration {
    /// Variants field.
    pub variants: Vec<Variant>,
}

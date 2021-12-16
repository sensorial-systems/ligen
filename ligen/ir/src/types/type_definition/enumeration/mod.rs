//! Enumeration representation.

mod variant;
pub use variant::*;

use crate::prelude::*;
use crate::{Attributes, Visibility, Identifier};

/// Enumeration representation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enumeration {
    /// Attributes field.
    pub attributes: Attributes,
    /// Enumeration visibility.
    pub visibility: Visibility,
    /// Enumeration identifier.
    pub identifier: Identifier,
    /// Variants field.
    pub variants: Vec<Variant>,
}

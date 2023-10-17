//! Enumeration representation.

#[cfg(any(test, feature = "mocks"))]
pub mod mock;

pub mod variant;
pub use variant::*;
use crate::{Attributes, Identifier, Path, Visibility};

use crate::prelude::*;

/// Enumeration representation.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enumeration {
    /// Structure attributes.
    pub attributes: Attributes,
    /// Structure visibility.
    pub visibility: Visibility,
    /// Structure identifier.
    pub identifier: Identifier,
    /// Variants field.
    pub variants: Vec<Variant>,
    /// Interfaces that this structure implements.
    pub interfaces: Vec<Path>

}

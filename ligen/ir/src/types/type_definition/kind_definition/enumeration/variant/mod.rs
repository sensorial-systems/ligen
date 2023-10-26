//! Enumeration variant representation.

use crate::prelude::*;
use crate::{Attributes, Identifier};

/// Enumeration representation.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Variant {
    /// Attributes field.
    pub attributes: Attributes,
    /// Variant identifier.
    pub identifier: Identifier
}

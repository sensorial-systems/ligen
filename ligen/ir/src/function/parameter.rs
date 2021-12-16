//! Function parameter.

use crate::prelude::*;
use crate::{Identifier, Type, Attributes};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
/// Parameter representation.
pub struct Parameter {
    /// Attributes.
    pub attributes: Attributes,
    /// Identifier.
    pub identifier: Identifier,
    /// Type.
    pub type_: Type,
}

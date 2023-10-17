use crate::{Identifier, Literal, Type};
use crate::prelude::*;

#[cfg(any(test, feature = "mocks"))]
pub mod mock;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
/// Constant Struct
pub struct Constant {
    /// Constant's identifier.
    pub identifier: Identifier,
    /// Constant's type.
    pub type_: Type,
    /// Constant's literal value.
    pub literal: Literal,
}

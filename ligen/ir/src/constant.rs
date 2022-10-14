use crate::{Identifier, Literal, Type};
use crate::prelude::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
/// Constant Struct
pub struct Constant {
    // TODO: Replace with fully qualified path.
    /// identifier field
    pub identifier: Identifier,
    /// type_ field
    pub type_: Type,
    /// literal field
    pub literal: Literal,
}

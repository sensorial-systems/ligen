use crate::{Identifier, Literal, Type};
use crate::prelude::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
/// Constant Struct
pub struct Constant {
    /// identifier field
    pub identifier: Identifier,
    /// type_ field
    pub type_: Type,
    /// literal field
    pub literal: Literal,
}

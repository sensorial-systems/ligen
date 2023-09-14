use crate::{Literal, Path, Type};
use crate::prelude::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
/// Constant Struct
pub struct Constant {
    /// Constant's path.
    pub path: Path,
    /// Constant's type.
    pub type_: Type,
    /// Constant's literal value.
    pub literal: Literal,
}

use crate::prelude::*;

use crate::{Attributes, Path, Type, Visibility};

pub mod parameter;
pub mod method;

pub use parameter::*;
pub use method::*;

/// Async structure.
#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct Async;

/// Function structure.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Function {
    /// Attributes field.
    pub attributes: Attributes,
    /// Visibility field.
    pub visibility: Visibility,
    /// Asyncness field.
    pub asyncness: Option<Async>,
    /// Function's absolute path.
    pub path: Path,
    /// Inputs field.
    pub inputs: Vec<Parameter>,
    /// Output field.
    pub output: Option<Type>,
}

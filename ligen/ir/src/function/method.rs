use crate::prelude::*;
use crate::{Async, Attributes, Mutability, Parameter, Path, Type, Visibility};

/// Method structure.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Method {
    /// Attributes field.
    pub attributes: Attributes,
    /// The owner of the method.
    pub owner: Type,
    /// The owner mutability.
    pub mutability: Mutability,
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

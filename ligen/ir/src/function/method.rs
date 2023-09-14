use crate::prelude::*;
use crate::{Synchrony, Attributes, Mutability, Parameter, Path, Type, Visibility};

/// Method structure.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Method {
    /// Attributes field.
    pub attributes: Attributes,
    /// The owner of the method.
    pub owner: Type,
    /// The owner mutability.
    pub mutability: Mutability,
    /// Visibility field.
    pub visibility: Visibility,
    /// Synchrony field.
    pub synchrony: Synchrony,
    /// Function's absolute path.
    pub path: Path,
    /// Inputs field.
    pub inputs: Vec<Parameter>,
    /// Output field.
    pub output: Option<Type>,
}

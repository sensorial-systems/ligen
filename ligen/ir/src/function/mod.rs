use crate::prelude::*;
use crate::{Attributes, Identifier, Parameter, Type, Visibility};

pub mod parameter;

/// Async structure.
#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct Async;

/// Method structure.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Method {
    pub mutability: Mutability,
    /// Method owner.
    pub owner: Type
}

// FIXME: Move this out of here.
/// Mutability.
#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum Mutability {
    Constant,
    Mutable
}

/// Function structure.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Function {
    /// Attributes field.
    pub attributes: Attributes,
    /// Visibility field.
    pub visibility: Visibility,
    /// Asyncness field.
    pub asyncness: Option<Async>,
    /// Method field.
    pub method: Option<Method>,
    /// Identifier field.
    pub identifier: Identifier,
    /// Inputs field.
    pub inputs: Vec<Parameter>,
    /// Output field.
    pub output: Option<Type>,
}

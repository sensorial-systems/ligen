use crate::prelude::*;

mod integer;
mod float;

pub use integer::*;
pub use float::*;
use crate::{Path, Identifier};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
/// Atomic Enum
pub enum Atomic {
    /// Integer variant
    Integer(Integer),
    /// Float variant
    Float(Float),
    /// Boolean variant
    Boolean,
    /// Character variant
    Character,
}

impl From<Integer> for Atomic {
    fn from(integer: Integer) -> Self {
        Self::Integer(integer)
    }
}

impl From<Float> for Atomic {
    fn from(float: Float) -> Self {
        Self::Float(float)
    }
}

impl From<Atomic> for Identifier {
    fn from(from: Atomic) -> Self {
        match from {
            Atomic::Boolean => "Boolean".into(),
            Atomic::Character => "Character".into(),
            Atomic::Float(float) => float.into(),
            Atomic::Integer(integer) => integer.into()
        }
    }
}

impl From<Atomic> for Path {
    fn from(from: Atomic) -> Self {
        Identifier::from(from).into()
    }
}

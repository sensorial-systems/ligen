use crate::{Atomic, Reference, Path, Identifier, Integer, Float, Generics};
use crate::prelude::*;
use std::ops::Deref;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
/// Type Enum
pub enum Type {
    /// Atomic variant
    Atomic(Atomic),
    /// Compound variant
    Compound(Path, Generics),
    /// Reference variant
    Reference(Reference),
}

impl Type {
    // FIXME: Rusty
    /// The Self type.
    pub fn self_type() -> Type {
        Type::from(Identifier::new("Self"))
    }

    /// Gets the path of the type without the reference.
    pub fn path(&self) -> Path {
        match self {
            Self::Reference(reference) => reference.type_.path(),
            Self::Compound(path, _) => path.clone(),
            Self::Atomic(atomic) => atomic.clone().into()
        }
    }

    /// Transforms Type::Reference to Type::Compound
    pub fn drop_reference(&self) -> Self {
        match self {
            Self::Reference(reference) => reference.type_.deref().clone(),
            _ => self.clone()
        }
    }
}

impl From<Identifier> for Type {
    fn from(identifier: Identifier) -> Self {
        Self::Compound(identifier.into(), Default::default())
    }
}

impl From<Path> for Type {
    fn from(path: Path) -> Self {
        Self::Compound(path, Default::default())
    }
}

impl From<Reference> for Type {
    fn from(reference: Reference) -> Self {
        Self::Reference(reference)
    }
}

impl From<Atomic> for Type {
    fn from(atomic: Atomic) -> Self {
        Self::Atomic(atomic)
    }
}

impl From<Integer> for Type {
    fn from(integer: Integer) -> Self {
        Self::Atomic(integer.into())
    }
}

impl From<Float> for Type {
    fn from(float: Float) -> Self {
        Self::Atomic(float.into())
    }
}

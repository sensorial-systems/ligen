use crate::{Primitive, Reference, Path, Identifier, Integer, Float, Generics};
use crate::prelude::*;
use std::ops::Deref;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
/// Type Enum
pub enum Type {
    /// Primitive variant
    Primitive(Primitive),
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
            Self::Primitive(primitive) => primitive.clone().into()
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

impl From<Primitive> for Type {
    fn from(primitive: Primitive) -> Self {
        Self::Primitive(primitive)
    }
}

impl From<Integer> for Type {
    fn from(integer: Integer) -> Self {
        Self::Primitive(integer.into())
    }
}

impl From<Float> for Type {
    fn from(float: Float) -> Self {
        Self::Primitive(float.into())
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let display = match &self {
            Type::Primitive(primitive)               => format!("{}", primitive),
            Type::Compound(compound, generics) => format!("{}{}", compound, generics),
            Type::Reference(reference)         => format!("{}", reference),
        };
        f.write_str(&display)
    }
}

use crate::{Primitive, Reference, Path, Identifier, Integer, Float, Generics};
use crate::prelude::*;
use std::ops::Deref;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
/// Type Enum
pub enum Type {
    /// Primitive variant
    Primitive(Primitive),
    /// Composite variant
    Composite(Path, Generics),
    /// Reference variant
    Reference(Reference),
}

impl Default for Type {
    fn default() -> Self {
        Self::Primitive(Primitive::Boolean)
    }
}

impl Type {
    /// Check if the `Type` is `Primitive`.
    pub fn is_primitive(&self) -> bool {
        match self {
            Self::Primitive(_) => true,
            _ => false
        }
    }

    /// Check if the `Type` is `Boolean`.
    pub fn is_boolean(&self) -> bool {
        match self {
            Self::Primitive(primitive) => primitive.is_boolean(),
            _ => false
        }
    }

    /// Check if the `Type` is `Character`.
    pub fn is_character(&self) -> bool {
        match self {
            Self::Primitive(primitive) => primitive.is_character(),
            _ => false
        }
    }

    /// Check if the `Type` is `Integer`.
    pub fn is_integer(&self) -> bool {
        match self {
            Self::Primitive(primitive) => primitive.is_integer(),
            _ => false
        }
    }

    /// Check if the `Type` is `UnsignedInteger`.
    pub fn is_unsigned_integer(&self) -> bool {
        match self {
            Self::Primitive(primitive) => primitive.is_unsigned_integer(),
            _ => false
        }
    }

    /// Check if the `Type` is `Float`.
    pub fn is_float(&self) -> bool {
        match self {
            Self::Primitive(primitive) => primitive.is_float(),
            _ => false
        }
    }

    /// Check if the `Type` is `String`.
    pub fn is_string(&self) -> bool {
        match self {
            Self::Composite(path, _) => path == &Path::from("String"), // TODO: Create a String type.
            _ => false
        }
    }

    /// Gets the path of the type without the reference.
    pub fn path(&self) -> Path {
        match self {
            Self::Reference(reference) => reference.type_.path(),
            Self::Composite(path, _) => path.clone(),
            Self::Primitive(primitive) => primitive.clone().into()
        }
    }

    /// Transforms Type::Reference to Type::Composite
    pub fn drop_reference(&self) -> Self {
        match self {
            Self::Reference(reference) => reference.type_.deref().clone(),
            _ => self.clone()
        }
    }
}

impl From<Identifier> for Type {
    fn from(identifier: Identifier) -> Self {
        Self::Composite(identifier.into(), Default::default())
    }
}

impl From<Path> for Type {
    fn from(path: Path) -> Self {
        Self::Composite(path, Default::default())
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
            Type::Composite(composite, generics) => format!("{}{}", composite, generics),
            Type::Reference(reference)         => format!("{}", reference),
        };
        f.write_str(&display)
    }
}

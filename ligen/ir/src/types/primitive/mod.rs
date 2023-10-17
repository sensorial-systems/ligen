use crate::prelude::*;

#[cfg(any(test, feature = "mocks"))]
pub mod mock;

pub mod integer;
pub mod float;

pub use integer::*;
pub use float::*;
use crate::{Identifier, Path};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
/// Primitive Enum
pub enum Primitive {
    /// Opaque variant
    Opaque,
    /// Integer variant
    Integer(Integer),
    /// Float variant
    Float(Float),
    /// Boolean variant
    Boolean,
    /// Character variant
    Character
}

impl Primitive {
    /// Checks if the `Primitive` is `Integer`.
    pub fn is_integer(&self) -> bool {
        match self {
            Self::Integer(integer) => !integer.is_unsigned(),
            _ => false
        }
    }

    /// Checks if the `Primitive` is `Float`.
    pub fn is_float(&self) -> bool {
        matches!(self, Self::Float(_))
    }

    /// Checks if the `Primitive` is `Boolean`.
    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean)
    }

    /// Checks if the `Primitive` is `Character`.
    pub fn is_character(&self) -> bool {
        matches!(self, Self::Character)
    }

    /// Checks if the `Primitive` is `UnsignedInteger`.
    pub fn is_unsigned_integer(&self) -> bool {
        match self {
            Self::Integer(integer) => integer.is_unsigned(),
            _ => false
        }
    }

    /// Returns true if the identifier is a primitive type.
    pub fn is_primitive<P: Into<Path>>(path: P) -> bool {
        let path = path.into();
        let identifier = path.last();
        match identifier.name.as_ref() {
            "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "i8" | "i16" | "i32" | "i64"
            | "i128" | "isize" | "f32" | "f64" | "bool" | "char" | "c_char" | "c_uchar" => true, // TODO: Is this Rusty? What are these c_char for?
            _ => false
        }
    }
}

impl From<Integer> for Primitive {
    fn from(integer: Integer) -> Self {
        Self::Integer(integer)
    }
}

impl From<Float> for Primitive {
    fn from(float: Float) -> Self {
        Self::Float(float)
    }
}

impl From<Primitive> for Identifier {
    fn from(primitive: Primitive) -> Self {
        match primitive {
            Primitive::Opaque => "Opaque".into(),
            Primitive::Boolean => "Boolean".into(),
            Primitive::Character => "Character".into(),
            Primitive::Float(float) => float.into(),
            Primitive::Integer(integer) => integer.into(),
        }
    }
}

impl From<Primitive> for Path {
    fn from(primitive: Primitive) -> Self {
        let primitive: Identifier = primitive.into();
        Path::from(primitive)
    }
}

impl std::fmt::Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let display = match &self {
            Primitive::Opaque           => "Opaque".into(),
            Primitive::Integer(integer) => format!("{}", integer),
            Primitive::Float(float)     => format!("{}", float),
            Primitive::Boolean          => "Boolean".into(),
            Primitive::Character        => "Character".into(),
        };
        f.write_str(&display)
    }
}

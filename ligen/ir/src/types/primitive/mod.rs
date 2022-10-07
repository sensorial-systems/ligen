use crate::prelude::*;

mod integer;
mod float;

pub use integer::*;
pub use float::*;
use crate::{Identifier, Path};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
/// Primitive Enum
pub enum Primitive {
    /// Integer variant
    Integer(Integer),
    /// Float variant
    Float(Float),
    /// Boolean variant
    Boolean,
    /// Character variant
    Character,
}

impl Primitive {
    /// Returns true if the identifier is a primitive type.
    pub fn is_primitive<P: Into<Path>>(path: P) -> bool {
        let path = path.into();
        let identifier = path.last();
        match identifier.name.as_ref() {
            "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "i8" | "i16" | "i32" | "i64"
            | "i128" | "isize" | "f32" | "f64" | "bool" | "char" | "c_char" | "c_uchar" => true,
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
            Primitive::Boolean => "bool".into(),
            Primitive::Character => "char".into(),
            Primitive::Float(float) => float.into(),
            Primitive::Integer(integer) => integer.into()
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
            Primitive::Integer(integer) => format!("{}", integer),
            Primitive::Float(float)     => format!("{}", float),
            Primitive::Boolean          => "bool".into(),
            Primitive::Character        => "char".into(),
        };
        f.write_str(&display)
    }
}

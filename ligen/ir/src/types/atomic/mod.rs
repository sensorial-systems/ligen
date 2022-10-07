use crate::prelude::*;

mod integer;
mod float;

pub use integer::*;
pub use float::*;
use crate::{Identifier, Path};

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

impl Atomic {
    /// Returns true if the identifier is an atomic type.
    pub fn is_atomic<P: Into<Path>>(path: P) -> bool {
        let path = path.into();
        let identifier = path.last();
        match identifier.name.as_ref() {
            "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "i8" | "i16" | "i32" | "i64"
            | "i128" | "isize" | "f32" | "f64" | "bool" | "char" | "c_char" | "c_uchar" => true,
            _ => false
        }
    }
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
    fn from(atomic: Atomic) -> Self {
        match atomic {
            Atomic::Boolean => "bool".into(),
            Atomic::Character => "char".into(),
            Atomic::Float(float) => float.into(),
            Atomic::Integer(integer) => integer.into()
        }
    }
}

impl From<Atomic> for Path {
    fn from(atomic: Atomic) -> Self {
        let atomic: Identifier = atomic.into();
        Path::from(atomic)
    }
}

impl std::fmt::Display for Atomic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let display = match &self {
            Atomic::Integer(integer) => format!("{}", integer),
            Atomic::Float(float)     => format!("{}", float),
            Atomic::Boolean          => "bool".into(),
            Atomic::Character        => "char".into(),
        };
        f.write_str(&display)
    }
}

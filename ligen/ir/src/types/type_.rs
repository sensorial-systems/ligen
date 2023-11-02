use crate::{Reference, Path, Identifier};
use crate::prelude::*;
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
/// Type Enum
pub enum Type {
    /// Type path.
    Path(Path),
    /// Type reference.
    Reference(Reference),
}

impl Type {
    /// Returns a new `Type` representing an opaque type.
    pub fn opaque() -> Self {
        Self::Path("Opaque".into())
    }

    /// Returns a new `Type` representing a boolean type.
    pub fn boolean() -> Self {
        Self::Path("Boolean".into())
    }

    /// Returns a new `Type` representing a character type.
    pub fn character() -> Self {
        Self::Path("Character".into())
    }

    /// Returns a new `Type` representing an 8-bit signed integer type.
    pub fn i8() -> Self {
        Self::Path("I8".into())
    }

    /// Returns a new `Type` representing a 16-bit signed integer type.
    pub fn i16() -> Self {
        Self::Path("I16".into())
    }

    /// Returns a new `Type` representing a 32-bit signed integer type.
    pub fn i32() -> Self {
        Self::Path("I32".into())
    }

    /// Returns a new `Type` representing a 64-bit signed integer type.
    pub fn i64() -> Self {
        Self::Path("I64".into())
    }

    /// Returns a new `Type` representing a 128-bit signed integer type.
    pub fn i128() -> Self {
        Self::Path("I128".into())
    }

    /// Returns a new `Type` representing an pointer-sized integer type.
    pub fn isize() -> Self {
        Self::Path("ISize".into())
    }

    /// Returns a new `Type` representing an 8-bit unsigned integer type.
    pub fn u8() -> Self {
        Self::Path("U8".into())
    }

    /// Returns a new `Type` representing a 16-bit unsigned integer type.
    pub fn u16() -> Self {
        Self::Path("U16".into())
    }

    /// Returns a new `Type` representing a 32-bit unsigned integer type.
    pub fn u32() -> Self {
        Self::Path("U32".into())
    }

    /// Returns a new `Type` representing a 64-bit unsigned integer type.
    pub fn u64() -> Self {
        Self::Path("U64".into())
    }

    /// Returns a new `Type` representing a 128-bit unsigned integer type.
    pub fn u128() -> Self {
        Self::Path("U128".into())
    }

    /// Returns a new `Type` representing an pointer-sized unsigned integer type.
    pub fn usize() -> Self {
        Self::Path("USize".into())
    }

    /// Returns a new `Type` representing a 16-bit floating-point type.
    pub fn f16() -> Self {
        Self::Path("F16".into())
    }

    /// Returns a new `Type` representing a 32-bit floating-point type.
    pub fn f32() -> Self {
        Self::Path("F32".into())
    }

    /// Returns a new `Type` representing a 64-bit floating-point type.
    pub fn f64() -> Self {
        Self::Path("F64".into())
    }

    /// Returns a new `Type` representing a 128-bit floating-point type.
    pub fn f128() -> Self {
        Self::Path("F128".into())
    }

    /// Returns a new `Type` representing a string type.
    pub fn string() -> Self {
        Self::Path("String".into())
    }
}

impl Default for Type {
    fn default() -> Self {
        Self::opaque()
    }
}

impl Type {
    /// Check if the `Type` is `Primitive`.
    pub fn is_primitive(&self) -> bool {
        self.is_boolean()
        || self.is_character()
        || self.is_float()
        || self.is_integer()
        || self.is_unsigned_integer()
    }

    pub fn is<T: Into<Self>>(&self, t: T) -> bool {
        self == &t.into()
    }

    /// Check if the `Type` is `Boolean`.
    pub fn is_boolean(&self) -> bool {
        self.is(Self::boolean())
    }

    /// Check if the `Type` is `Character`.
    pub fn is_character(&self) -> bool {
        self.is(Self::character())
    }

    /// Check if the `Type` is a number.
    pub fn is_number(&self) -> bool {
        self.is_integer() || self.is_float()
    }    

    /// Check if the `Type` is integer.
    pub fn is_integer(&self) -> bool {
        self.is(Self::i8()) || self.is(Self::i16()) || self.is(Self::i32()) || self.is(Self::i64()) || self.is(Self::i128())
    }

    /// Check if the `Type` is `UnsignedInteger`.
    pub fn is_unsigned_integer(&self) -> bool {
        self.is(Self::u8()) || self.is(Self::u16()) || self.is(Self::u32()) || self.is(Self::u64()) || self.is(Self::u128())
    }

    /// Check if the `Type` is `Float`.
    pub fn is_float(&self) -> bool {
        self.is(Self::f32()) || self.is(Self::f64()) || self.is(Self::f128())
    }

    /// Check if the `Type` is `String`.
    pub fn is_string(&self) -> bool {
        self.is(Self::string())
    }

    /// Transforms Type::Reference to Type::Path
    pub fn drop_reference(&self) -> Self {
        match self {
            Self::Reference(reference) => reference.type_.deref().clone(),
            _ => self.clone()
        }
    }
}

impl From<&str> for Type {
    fn from(value: &str) -> Self {
        Self::Path(value.into())
    }
}

impl From<Identifier> for Type {
    fn from(identifier: Identifier) -> Self {
        Self::Path(identifier.into())
    }
}

impl From<Path> for Type {
    fn from(path: Path) -> Self {
        Self::Path(path)
    }
}

impl From<Reference> for Type {
    fn from(reference: Reference) -> Self {
        Self::Reference(reference)
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let display = match &self {
            Type::Path(path) => format!("{}", path),
            Type::Reference(reference) => format!("{}", reference),
        };
        f.write_str(&display)
    }
}

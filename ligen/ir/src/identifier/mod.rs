#[cfg(any(test, feature = "mocks"))]
pub mod mock;

pub mod naming_convention;
use is_tree::IsPathSegment;
pub use naming_convention::*;

use crate::path::PathSegment;
use crate::{prelude::*, Mutability};

/// Identifier structure
#[derive(Clone, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Display, Serialize, Deserialize, JsonSchema)]
#[display(fmt = "{}", name)]
pub struct Identifier {
    /// Name field of Identifier
    pub name: String,
}

impl PartialEq<Identifier> for &str {
    fn eq(&self, other: &Identifier) -> bool {
        self.eq(&other.name)
    }
}

impl Identifier {
    /// Create a new Identifier
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        Self { name }
    }

    /// Returns a new `Identifier` representing a reference type.
    pub fn reference(mutability: Mutability) -> Self {
        match mutability {
            Mutability::Constant => Self::constant_reference(),
            Mutability::Mutable => Self::mutable_reference(),
        }
    }

    /// Returns a new `Identifier` representing a mutable reference type.
    pub fn mutable_reference() -> Self {
        "MutableReference".into()
    }

    /// Returns a new `Identifier` representing a constant reference type.
    pub fn constant_reference() -> Self {
        "Reference".into()
    }

    /// Returns a new `Identifier` representing a union type.
    pub fn union() -> Self {
        "Union".into()
    }

    /// Returns a new `Identifier` representing a variadic type.
    pub fn variadic() -> Self {
        "Variadic".into()
    }

    /// Returns a new `Identifier` representing a tuple type.
    pub fn tuple() -> Self {
        "Tuple".into()
    }

    /// Returns a new `Identifier` representing a dictionary type.
    pub fn dictionary() -> Self {
        "Dictionary".into()
    }

    /// Returns a new `Identifier` representing a slice type.
    pub fn slice() -> Self {
        "Slice".into()
    }

    /// Returns a new `Identifier` representing an array type.
    pub fn array() -> Self {
        "Array".into()
    }

    /// Returns a new `Identifier` representing a vector type.
    pub fn vector() -> Self {
        "Vector".into()
    }

    /// Returns a new `Identifier` representing a date type.
    pub fn date_time() -> Self {
        "DateTime".into()
    }

    /// Returns a new `Identifier` representing an Option type.
    pub fn option() -> Self {
        "Option".into()
    }

    /// Returns a new `Identifier` representing an opaque type.
    pub fn opaque() -> Self {
        "Opaque".into()
    }

    /// Returns a new `Identifier` representing a boolean type.
    pub fn boolean() -> Self {
        "Boolean".into()
    }

    /// Returns a new `Identifier` representing a character type.
    pub fn character() -> Self {
        "Character".into()
    }

    /// Returns a new `Identifier` representing an 8-bit signed integer type.
    pub fn i8() -> Self {
        "I8".into()
    }

    /// Returns a new `Identifier` representing a 16-bit signed integer type.
    pub fn i16() -> Self {
        "I16".into()
    }

    /// Returns a new `Identifier` representing a 32-bit signed integer type.
    pub fn i32() -> Self {
        "I32".into()
    }

    /// Returns a new `Identifier` representing a 64-bit signed integer type.
    pub fn i64() -> Self {
        "I64".into()
    }

    /// Returns a new `Identifier` representing a 128-bit signed integer type.
    pub fn i128() -> Self {
        "I128".into()
    }

    /// Returns a new `Identifier` representing an pointer-sized integer type.
    pub fn isize() -> Self {
        "ISize".into()
    }

    /// Returns a new `Identifier` representing an 8-bit unsigned integer type.
    pub fn u8() -> Self {
        "U8".into()
    }

    /// Returns a new `Identifier` representing a 16-bit unsigned integer type.
    pub fn u16() -> Self {
        "U16".into()
    }

    /// Returns a new `Identifier` representing a 32-bit unsigned integer type.
    pub fn u32() -> Self {
        "U32".into()
    }

    /// Returns a new `Identifier` representing a 64-bit unsigned integer type.
    pub fn u64() -> Self {
        "U64".into()
    }

    /// Returns a new `Identifier` representing a 128-bit unsigned integer type.
    pub fn u128() -> Self {
        "U128".into()
    }

    /// Returns a new `Identifier` representing an pointer-sized unsigned integer type.
    pub fn usize() -> Self {
        "USize".into()
    }

    /// Returns a new `Identifier` representing a 16-bit floating-point type.
    pub fn f16() -> Self {
        "F16".into()
    }

    /// Returns a new `Identifier` representing a 32-bit floating-point type.
    pub fn f32() -> Self {
        "F32".into()
    }

    /// Returns a new `Identifier` representing a 64-bit floating-point type.
    pub fn f64() -> Self {
        "F64".into()
    }

    /// Returns a new `Identifier` representing a 128-bit floating-point type.
    pub fn f128() -> Self {
        "F128".into()
    }

    /// Returns a new `Identifier` representing a string type.
    pub fn string() -> Self {
        "String".into()
    }

    /// Returns a new `Identifier` representing the `root` keyword.
    pub fn root() -> Self {
        "root".into()
    }

    /// Returns a new `Identifier` representing the `super` keyword.
    pub fn super_() -> Self {
        "super".into()
    }

    /// Returns a new `Identifier` representing the `self` keyword.
    pub fn self_() -> Self {
        "self".into()
    }
    
}

impl From<Identifier> for String {
    fn from(value: Identifier) -> Self {
        value.name
    }
}

impl PartialEq<&str> for Identifier {
    fn eq(&self, other: &&str) -> bool {
        self.name == *other
    }
}

impl From<&str> for Identifier {
    fn from(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}

impl From<String> for Identifier {
    fn from(name: String) -> Self {
        name.as_str().into()
    }
}

impl From<PathSegment> for Identifier {
    fn from(value: PathSegment) -> Self {
        value.identifier
    }
}

impl IsPathSegment for Identifier {
    fn root() -> Self {
        Self::new("root")
    }

    fn self_() -> Self {
        Self::new("self")
    }

    fn super_() -> Self {
        Self::new("super")
    }
}

impl std::ops::Add<&str> for &Identifier {
    type Output = Identifier;

    fn add(self, other: &str) -> Identifier {
        Identifier::new(format!("{}{}", self.name, other))
    }
}

impl std::ops::Add<&Identifier> for &str {
    type Output = Identifier;

    fn add(self, other: &Identifier) -> Identifier {
        Identifier::new(format!("{}{}", self, other.name))
    }
}

impl std::ops::Add<Identifier> for &Identifier {
    type Output = Identifier;

    fn add(self, other: Identifier) -> Identifier {
        Identifier::new(format!("{}{}", self.name, other.name))
    }
}
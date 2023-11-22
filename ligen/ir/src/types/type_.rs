use is_tree::{IntoIterTypeMut, TypeIteratorMut};

use crate::{Path, Identifier, PathSegment, Mutability};
use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
/// Type structure.
pub struct Type {
    /// Type path.
    pub path: Path
}

// TODO: Move these constructors to its all structures? Reference, Vector, String, etc... And make them convertable to Type.
impl Type {
    /// Returns a new `Type` representing a mutable reference type.
    pub fn mutable_reference(type_: impl Into<Type>) -> Self {
        PathSegment::new(Identifier::mutable_reference(), type_.into()).into()
    }

    /// Returns a new `Type` representing a reference type.
    pub fn reference(mutability: Mutability, type_: impl Into<Type>) -> Self {
        PathSegment::new(Identifier::reference(mutability), type_.into()).into()
    }

    /// Returns a new `Type` representing a constant reference type.
    pub fn constant_reference(type_: impl Into<Type>) -> Self {
        PathSegment::new(Identifier::constant_reference(), type_.into()).into()
    }

    /// Returns a new `Type` representing a vector type.
    pub fn vector(type_: impl Into<Type>) -> Self {
        Path::from(PathSegment::new(Identifier::vector(), type_.into())).into()
    }

    /// Returns a new `Type`representing an union type.
    pub fn union(types: Vec<Type>) -> Self {
        Path::from(PathSegment::new(Identifier::union(), types)).into()
    }

    /// Returns a new `Type` representing a variadic type.
    pub fn variadic(type_: impl Into<Type>) -> Self {
        Path::from(PathSegment::new(Identifier::variadic(), type_.into())).into()
    }

    /// Returns a new `Type` representing a dictionary type.
    pub fn tuple(types: Vec<Type>) -> Self {
        Path::from(PathSegment::new(Identifier::tuple(), types)).into()
    }

    /// Returns a new `Type` representing a option type.
    pub fn option(type_: impl Into<Type>) -> Self {
        Path::from(PathSegment::new(Identifier::option(), type_.into())).into()
    }

    /// Returns a new `Type` representing an opaque type.
    pub fn opaque() -> Self {
        Identifier::opaque().into()
    }

    /// Returns a new `Type` representing a boolean type.
    pub fn boolean() -> Self {
        Identifier::boolean().into()
    }

    /// Returns a new `Type` representing a character type.
    pub fn character() -> Self {
        Identifier::character().into()
    }

    /// Returns a new `Type` representing an 8-bit signed integer type.
    pub fn i8() -> Self {
        Identifier::i8().into()
    }

    /// Returns a new `Type` representing a 16-bit signed integer type.
    pub fn i16() -> Self {
        Identifier::i16().into()
    }

    /// Returns a new `Type` representing a 32-bit signed integer type.
    pub fn i32() -> Self {
        Identifier::i32().into()
    }

    /// Returns a new `Type` representing a 64-bit signed integer type.
    pub fn i64() -> Self {
        Identifier::i64().into()
    }

    /// Returns a new `Type` representing a 128-bit signed integer type.
    pub fn i128() -> Self {
        Identifier::i128().into()
    }

    /// Returns a new `Type` representing an pointer-sized integer type.
    pub fn isize() -> Self {
        Identifier::isize().into()
    }

    /// Returns a new `Type` representing an 8-bit unsigned integer type.
    pub fn u8() -> Self {
        Identifier::u8().into()
    }

    /// Returns a new `Type` representing a 16-bit unsigned integer type.
    pub fn u16() -> Self {
        Identifier::u16().into()
    }

    /// Returns a new `Type` representing a 32-bit unsigned integer type.
    pub fn u32() -> Self {
        Identifier::u32().into()
    }

    /// Returns a new `Type` representing a 64-bit unsigned integer type.
    pub fn u64() -> Self {
        Identifier::u64().into()
    }

    /// Returns a new `Type` representing a 128-bit unsigned integer type.
    pub fn u128() -> Self {
        Identifier::u128().into()
    }

    /// Returns a new `Type` representing an pointer-sized unsigned integer type.
    pub fn usize() -> Self {
        Identifier::usize().into()
    }

    /// Returns a new `Type` representing a 16-bit floating-point type.
    pub fn f16() -> Self {
        Identifier::f16().into()
    }

    /// Returns a new `Type` representing a 32-bit floating-point type.
    pub fn f32() -> Self {
        Identifier::f32().into()
    }

    /// Returns a new `Type` representing a 64-bit floating-point type.
    pub fn f64() -> Self {
        Identifier::f64().into()
    }

    /// Returns a new `Type` representing a 128-bit floating-point type.
    pub fn f128() -> Self {
        Identifier::f128().into()
    }

    /// Returns a new `Type` representing a string type.
    pub fn string() -> Self {
        Identifier::string().into()
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

    /// Check if the `Type` is `MutableReference`
    pub fn is_mutable_reference(&self) -> bool {
        self.path.last().identifier == Identifier::mutable_reference()
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
}

impl From<PathSegment> for Type {
    fn from(path_segment: PathSegment) -> Self {
        let path = path_segment.into();
        Self { path }
    }
}

impl From<&str> for Type {
    fn from(value: &str) -> Self {
        let path = value.into();
        Self { path }
    }
}

impl From<Identifier> for Type {
    fn from(identifier: Identifier) -> Self {
        let path = identifier.into();
        Self { path }
    }
}

impl From<Path> for Type {
    fn from(path: Path) -> Self {
        Self { path }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.path.to_string())
    }
}

impl IntoIterTypeMut<Type> for Type {
    fn into_type_iterator<'a>(&'a mut self) -> TypeIteratorMut<'a, Type> {
        // FIXME: Is this safe?
        let myself = unsafe { &mut *(self as *mut Self) };
        let mut stack = vec![myself];
        stack.extend(self.path.into_type_iterator());
        stack.into()
    }
}
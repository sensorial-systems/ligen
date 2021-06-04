use super::type_::Type;

#[derive(Debug, PartialEq, Clone)]
/// Borrow Enum
pub enum Borrow {
    /// Constant variant
    Constant(Box<Type>),
    /// Mutable variant
    Mutable(Box<Type>),
}

impl Borrow {
    /// Returns true if the reference is constant.
    pub fn is_constant(&self) -> bool {
        match self {
            Self::Mutable(_) => false,
            Self::Constant(_) => true,
        }
    }

    /// Gets the underlying type.
    pub fn type_mut(&mut self) -> &mut Type {
        match self {
            Self::Mutable(type_) => &mut *type_,
            Self::Constant(type_) => &mut *type_
        }
    }

    /// Gets the underlying type.
    pub fn type_(&self) -> &Type {
        match self {
            Self::Mutable(type_) => &*type_,
            Self::Constant(type_) => &*type_
        }
    }
}

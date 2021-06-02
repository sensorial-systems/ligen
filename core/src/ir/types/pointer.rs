use crate::ir::Type;

#[derive(Debug, PartialEq, Clone)]
/// Pointer Enum
pub enum Pointer {
    /// Constant variant
    Constant(Box<Type>),
    /// Mutable variant
    Mutable(Box<Type>),
}

impl Pointer {
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

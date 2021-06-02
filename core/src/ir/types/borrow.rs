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
    /// Gets the underlying type.
    pub fn type_mut(&mut self) -> &mut Type {
        match self {
            Self::Mutable(type_) => &mut *type_,
            Self::Constant(type_) => &mut *type_
        }
    }
}

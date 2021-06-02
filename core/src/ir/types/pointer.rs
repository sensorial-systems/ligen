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
    /// Gets the underlying type.
    pub fn type_mut(&mut self) -> &mut Type {
        match self {
            Self::Mutable(type_) => &mut *type_,
            Self::Constant(type_) => &mut *type_
        }
    }
}

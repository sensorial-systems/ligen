use crate::ir::Borrow;
use crate::ir::Pointer;
use crate::ir::Type;

#[derive(Debug, PartialEq, Clone)]
/// Reference Enum
pub enum Reference {
    /// Borrow variant
    Borrow(Borrow),
    /// Pointer variant
    Pointer(Pointer),
}

impl Reference {
    /// Returns true if the reference is constant.
    pub fn is_constant(&self) -> bool {
        match self {
            Self::Pointer(pointer) => pointer.is_constant(),
            Self::Borrow(borrow) => borrow.is_constant(),
        }
    }

    /// Gets the underlying type.
    pub fn type_mut(&mut self) -> &mut Type {
        match self {
            Self::Pointer(pointer) => pointer.type_mut(),
            Self::Borrow(borrow) => borrow.type_mut()
        }
    }

    /// Gets the underlying type.
    pub fn type_(&self) -> &Type {
        match self {
            Self::Pointer(pointer) => pointer.type_(),
            Self::Borrow(borrow) => borrow.type_()
        }
    }
}

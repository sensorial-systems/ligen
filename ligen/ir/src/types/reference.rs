use crate::{Type, Mutability};
use crate::prelude::*;

/// Reference kind.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum ReferenceKind {
    /// Borrow reference, denoted with &.
    Borrow,
    /// Pointer reference, denoted with *.
    Pointer
}

/// Reference representation.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Reference {
    /// Indicates the reference kind.
    pub kind: ReferenceKind,
    /// Mutability.
    pub mutability: Mutability,
    /// The type being referenced.
    pub type_: Box<Type>
}

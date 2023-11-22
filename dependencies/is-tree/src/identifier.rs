use std::{hash::Hash, fmt::Display};

pub enum Identifier<T> {
    Root,
    Self_,
    Super,
    Other(T)
}

pub trait IsIdentifier: PartialEq + Eq + Hash + Clone + Display {
    fn root() -> Self;
    fn self_() -> Self;
    fn super_() -> Self;
    fn kind(&self) -> Identifier<&Self> {
        if Self::root().eq(self) {
            Identifier::Root
        } else if Self::self_().eq(self) {
            Identifier::Self_
        } else if Self::super_().eq(self) {
            Identifier::Super
        } else {
            Identifier::Other(self)
        }
    }
}

impl IsIdentifier for String {
    fn root() -> Self {
        "root".to_string()
    }
    fn self_() -> Self {
        "self".to_string()
    }
    fn super_() -> Self {
        "super".to_string()
    }
}

pub trait HasIdentifier {
    type Identifier: IsIdentifier;
    fn identifier(&self) -> &Self::Identifier;
}

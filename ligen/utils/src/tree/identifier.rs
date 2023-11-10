use std::hash::Hash;

pub trait Identifier {
    type Identifier: PartialEq + Eq + Hash + Clone;
    fn identifier(&self) -> &Self::Identifier;
}

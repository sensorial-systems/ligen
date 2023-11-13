use std::hash::Hash;

pub trait HasIdentifier {
    type Identifier: PartialEq + Eq + Hash + Clone;
    fn identifier(&self) -> &Self::Identifier;
}

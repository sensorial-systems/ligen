pub trait Identifier {
    type Identifier: PartialEq;
    fn identifier(&self) -> &Self::Identifier;
}

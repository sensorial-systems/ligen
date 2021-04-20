use crate::prelude::*;

use crate::ir::Literal;
use crate::ir::Identifier;

use syn::{AttributeArgs, ItemFn};
use proc_macro2::TokenStream;

/// Attribute Enum
#[derive(Debug)]
pub enum Attribute {
   Literal(Literal),
   Named(Identifier, Literal),
   Group(Identifier, Vec<Attribute>),
}

#[derive(Shrinkwrap, Default)]
#[shrinkwrap(mutable)]
pub struct Attributes {
    pub attributes: Vec<Attribute>
}

impl From<syn::AttributeArgs> for Attributes {
    fn from(_args: syn::AttributeArgs) -> Attributes {
        todo!()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn literal() {}

    #[test]
    fn named() {}

    #[test]
    fn group() {}
}
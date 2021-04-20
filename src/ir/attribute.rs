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
   Group(Identifier, Attributes),
}

#[derive(Shrinkwrap, Default)]
#[shrinkwrap(mutable)]
pub struct Attributes {
    pub attributes: Vec<Attribute>
}

impl From<syn::AttributeArgs> for Attributes {
    fn from(attribute_args: syn::AttributeArgs) -> Self {
        todo!()
    }
}

impl From<syn::MetaList> for Attributes {
    fn from(meta_list: syn::MetaList) -> Self {
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
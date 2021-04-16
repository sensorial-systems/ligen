use crate::ir::Literal;
use crate::ir::Identifier;

extern crate darling;
extern crate syn;

extern crate proc_macro;

use darling::{FromVariant, FromMeta, *};
use syn::{AttributeArgs, ItemFn};
use proc_macro::TokenStream;

/// Attribute Enum
//#[derive(Debug, FromMeta)]
//#[darling(default)]
//pub enum Attribute {
   // Literal(Literal),
  //  Named(Literal),
 //   Group(Vec<Attribute>),
//}


#[derive(Debug, FromMeta)]
pub struct Attribute {
    #[darling(default)]
    identifier: String,
 //   literal: Literal,
}



impl Attribute {
    pub fn parse_args(args : &syn::AttributeArgs) -> Attribute {
        match Attribute::from_list(&args) {
        Ok(v) => v,
        Err(e) => { Attribute {identifier: String::from("")} }
    }

    }
}

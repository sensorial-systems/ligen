//! This crate provides core functionalities for ligen.

#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unsafe_code)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]

pub mod ir;
mod prelude;

use std::convert::TryFrom;

use crate::ir::{Attribute, Attributes};
use ir::Identifier;
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use syn::AttributeArgs;
/// `ligen` entry-point called by `#[ligen]`.
pub fn ligen(args: AttributeArgs, item: TokenStream) -> TokenStream {
    let args = Attributes::try_from(args).expect("Failed to parse AttributeArgs");

    let mut stream: TokenStream = TokenStream::new();

    args.attributes.into_iter().for_each(|x| match x {
        Attribute::Literal(lit) => {
            let id = Identifier {
                name: String::from(format!("ligen_{}", lit)),
            };
            stream.append_all(quote! {#[#id]})
        }
        Attribute::Group(ident, group) => group.attributes.into_iter().for_each(|x| match x {
            Attribute::Literal(lit) => {
                let id = Identifier {
                    name: String::from(format!("ligen_{}", ident.name)),
                };
                stream.append_all(quote! {#[#id(#lit)]})
            }
            Attribute::Named(ident2, lit) => {
                let id = Identifier {
                    name: String::from(format!("ligen_{}", ident.name)),
                };
                stream.append_all(quote! {#[#id(#ident2 = #lit)]})
            }
            _ => panic!("panic"),
        }),
        _ => panic!("panic2"),
    });

    println!("stream: {:#?}", stream);

    quote! {
        #stream
        #item
    }
}

#[cfg(test)]
mod test {
    use quote::quote;
    use std::convert::TryInto;
    use syn::parse_quote::parse;

    // #[test]
    //fn ligen() {
    //     parse::<syn::Type>(quote! {#[ligen()]})
    //        .try_into()
    //        .expect("Failed to convert from syn::Type")
    // }
}

use crate::generator::Context;
use crate::ir::{Attribute, Literal, Identifier, Attributes, ProceduralMacroAttributes};

use proc_macro2::TokenStream;
use quote::TokenStreamExt;
use quote::quote;

use std::convert::TryFrom;

const PREFIX: &'static str = "ligen_";

/// `ligen` entry-point called by `#[ligen]`.
pub fn ligen(_context: Context, args: TokenStream, item: TokenStream) -> TokenStream {
    let args = Attributes::try_from(args).expect("Failed to parse Attributes.");

    let attributes = args.attributes.into_iter().map(to_ligen_macro).fold(
        TokenStream::new(),
        |mut attributes, macro_attribute| {
            let procedural_macro_attributes = ProceduralMacroAttributes::from(macro_attribute);
            attributes.append_all(quote! { #procedural_macro_attributes });
            attributes
        },
    );

    let tokenstream = quote! {
        #attributes
        #item
    };

    tokenstream
}

/// Convert Attribute to a Ligen Macro attribute
fn to_ligen_macro(attribute: Attribute) -> Attribute {
    match attribute {
        Attribute::Literal(literal) => {
            Attribute::Literal(Literal::String(format!("{}{}", PREFIX, literal)))
        }
        Attribute::Named(ident, lit) => Attribute::Named(ident, lit),
        Attribute::Group(ident, group) => Attribute::Group(
            Identifier::new(format!("{}{}", PREFIX, ident.name).as_str()),
            Attributes {
                attributes: group
                    .attributes
                    .into_iter()
                    .filter_map(|x| {
                        if let Attribute::Named(ident, lit) = x {
                            Some(Attribute::Named(ident, lit))
                        } else {
                            None
                        }
                    })
                    .collect(),
            },
        ),
    }
}

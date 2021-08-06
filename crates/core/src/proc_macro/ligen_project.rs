use crate::ir::{Attributes, Attribute, Identifier};
use proc_macro2::TokenStream;
use quote::TokenStreamExt;
use quote::quote;
use std::convert::TryFrom;

/// `ligen_project` proc_macro function called by `ligen_project!()`
pub fn ligen_project(attributes: TokenStream) -> TokenStream {
    let attributes = Attributes::try_from(attributes).expect("Failed to parse Attributes.");

    let mut output = TokenStream::new();
    attributes.attributes
        .into_iter()
        .for_each(|attribute| output.append_all(to_project_tokens(&attribute)));

    output
}
/// Function to get a TokenStream of Attribute as a ligen project generator proc_macro
fn to_project_tokens(attribute: &Attribute) -> TokenStream {
    match attribute {
        Attribute::Literal(lit) => {
            let ident = Identifier::new(format!("ligen_{}", &lit.to_string()).as_str());

            quote! {#ident!();}
        }
        Attribute::Named(_, _) => panic!("Named variant should only be used inside groups"),
        Attribute::Group(ident, group) => {
            let mut gp = TokenStream::new();
            group
                .attributes
                .clone()
                .into_iter()
                .enumerate()
                .for_each(|x| {
                    if let (index, Attribute::Literal(lit)) = x {
                        let name = Identifier::new(&lit.to_string());
                        gp.append_all(quote! {#name});
                        if index + 1 < group.attributes.len() {
                            gp.append_all(quote! {, })
                        }
                    } else {
                        panic!("Group contains Named variant")
                    }
                });

            let ident = Identifier::new(format!("ligen_{}", &ident.name).as_str());
            quote! {#ident!(#gp);}
        }
    }
}

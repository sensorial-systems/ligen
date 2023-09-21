//! Structure representation.

pub mod field;
pub use field::*;

use crate::prelude::*;
use ligen_ir::Structure;
use ligen_parsing::Parser;

pub struct StructureParser;

impl Parser<proc_macro2::TokenStream> for StructureParser {
    type Output = Structure;
    fn parse(&self, tokenstream: proc_macro2::TokenStream) -> Result<Self::Output> {
        syn::parse2::<syn::ItemStruct>(tokenstream.into())
            .map_err(|_| "Failed to parse to Structure.".into())
            .and_then(|item| SynItemStruct::from(item).try_into())
    }
}

impl TryFrom<SynItemStruct> for Structure {
    type Error = Error;
    fn try_from(SynItemStruct(structure): SynItemStruct) -> Result<Self> {
        let mut fields = Vec::new();
        for field in structure.fields {
            fields.push(SynField(field).try_into()?);
        }
        Ok(Self { fields })
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::parse_quote::parse;
    use std::convert::TryFrom;
    use ligen_ir::{Field, Type, Primitive, Integer, Visibility, Structure};
    use crate::prelude::SynItemStruct;

    #[test]
    fn structure() {
        let structure = parse::<syn::ItemStruct>(quote! {
            struct Structure {
                integer: i32
            }
        });
        assert_eq!(
            Structure::try_from(SynItemStruct(structure)).expect("Failed to convert structure."),
            Structure {
                fields: vec! [
                    Field {
                        attributes: Default::default(),
                        visibility: Visibility::Private,
                        identifier: Some("integer".into()),
                        type_: Type::Primitive(Primitive::Integer(Integer::I32))
                    }
                ]
            }
        );
    }
}

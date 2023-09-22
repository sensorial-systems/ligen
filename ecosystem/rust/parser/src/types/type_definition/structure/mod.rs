//! Structure representation.

pub mod field;

pub use field::*;

use crate::prelude::*;
use ligen_ir::Structure;
use ligen_parsing::Parser;

pub struct StructureParser;

impl Parser<proc_macro::TokenStream> for StructureParser {
    type Output = Structure;
    fn parse(&self, token_stream: proc_macro::TokenStream) -> Result<Self::Output> {
        self.parse(proc_macro2::TokenStream::from(token_stream))
    }
}

impl Parser<proc_macro2::TokenStream> for StructureParser {
    type Output = Structure;
    fn parse(&self, tokenstream: proc_macro2::TokenStream) -> Result<Self::Output> {
        syn::parse2::<syn::ItemStruct>(tokenstream.into())
            .map_err(|e| Error::Message(format!("Failed to parse to structure: {:?}", e)))
            .and_then(|structure| self.parse(structure))
    }
}

impl Parser<syn::ItemStruct> for StructureParser {
    type Output = Structure;
    fn parse(&self, structure: syn::ItemStruct) -> Result<Self::Output> {
        let mut fields = Vec::new();
        for field in structure.fields {
            fields.push(FieldParser.parse(field)?);
        }
        Ok(Self::Output { fields })
    }
}

#[cfg(test)]
mod tests {
    use ligen_ir::{Field, Type, Primitive, Integer, Visibility, Structure};
    use ligen_parsing::Parser;
    use crate::types::structure::StructureParser;
    use crate::prelude::*;

    #[test]
    fn structure() -> Result<()> {
        let structure = quote! {
            struct Structure {
                integer: i32
            }
        };
        assert_eq!(
            StructureParser.parse(structure)?,
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
        Ok(())
    }
}

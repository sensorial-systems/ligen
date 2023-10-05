//! Structure representation.

pub mod field;

pub use field::*;

use crate::prelude::*;
use ligen::ir::Structure;
use ligen::parsing::parser::Parser;

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
    use crate::types::structure::StructureParser;
    use crate::prelude::*;

    use ligen::parsing::assert::*;
    use ligen::ir::structure::mock;

    #[test]
    fn structure() -> Result<()> {
        assert_eq(StructureParser, mock::structure(), quote! {
            struct Structure {
                integer: i32
            }
        })
    }
}

use ligen::{ir::Type, parser::ParserConfig};
use quote::ToTokens;
use syn::{TypeArray, TypeSlice};
use crate::{literal::LiteralParser, mutability::MutabilityParser, prelude::*};
use ligen::parser::Parser;
use crate::path::PathParser;

#[derive(Default)]
pub struct TypeParser {
    pub mutability_parser: MutabilityParser,
    pub literal_parser: LiteralParser
}

impl TypeParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Parser<syn::Ident> for TypeParser {
    type Output = Type;
    fn parse(&self, input: syn::Ident, config: &ParserConfig) -> Result<Self::Output> {
        Ok(PathParser::default().parse(input, config)?.into())
    }
}

impl Parser<syn::Path> for TypeParser {
    type Output = Type;
    fn parse(&self, path: syn::Path, config: &ParserConfig) -> Result<Self::Output> {
        let mut path = PathParser::default().parse(path, config)?;
        if path.segments.len() == 1 {
            let segment = path.first_mut();
            match segment.identifier.name.as_str() {
                "i8"  | "i16" | "i32" | "i64" | "i128" |
                "u8"  | "u16" | "u32" | "u64" | "u128" |
                "f16" | "f32" | "f64" | "f128" =>
                    segment
                        .identifier
                        .name
                        .replace_range(..1, &segment.identifier.name[..1].to_uppercase()),
                "usize" | "isize" =>
                    segment
                        .identifier
                        .name
                        .replace_range(..2, &segment.identifier.name[..2].to_uppercase()),
                "char" => segment.identifier.name = "Character".into(),
                "bool" => segment.identifier.name = "Boolean".into(),
                _ => ()
            }
        }
        Ok(path.into())
    }
}

impl Parser<syn::Type> for TypeParser {
    type Output = Type;
    fn parse(&self, syn_type: syn::Type, config: &ParserConfig) -> Result<Self::Output> {
        if let syn::Type::Path(syn::TypePath { path, .. }) = syn_type {
            Ok(self.parse(path, config)?)
        } else {
            match syn_type {
                syn::Type::Reference(syn::TypeReference { elem, mutability, .. }) |
                syn::Type::Ptr(syn::TypePtr { elem, mutability, .. }) => {
                    let mutability = self.mutability_parser.parse(mutability, config)?;
                    let type_ = TypeParser::new().parse(*elem, config)?;
                    Ok(Type::reference(mutability, type_))
                },
                syn::Type::Slice(TypeSlice { elem, .. }) => {
                    let type_ = TypeParser::new().parse(*elem, config)?;
                    Ok(Type::slice(type_))
                },
                syn::Type::Array(TypeArray { elem, len, .. }) => {
                    let len = self.literal_parser.parse(len, config)?;
                    let len = len.into_integer().map_err(|_| Error::Message("Array length literal isn't an integer.".into()))? as usize;
                    let type_ = TypeParser::new().parse(*elem, config)?;
                    Ok(Type::array(type_, len))
                },
                _ => Err(Error::Message(format!("\"{}\" not supported. Only Path, Reference and Ptr Types are currently supported", syn_type.to_token_stream()))),
            }
        }
    }
}

impl Parser<proc_macro::TokenStream> for TypeParser {
    type Output = Type;
    fn parse(&self, input: proc_macro::TokenStream, config: &ParserConfig) -> Result<Self::Output> {
        self.parse(proc_macro2::TokenStream::from(input), config)
    }
}

impl Parser<proc_macro2::TokenStream> for TypeParser {
    type Output = Type;
    fn parse(&self, input: proc_macro2::TokenStream, config: &ParserConfig) -> Result<Self::Output> {
        syn::parse2::<syn::Type>(input)
            .map_err(|e| Error::Message(format!("Failed to parse type: {}", e)))
            .and_then(|syn_type| self.parse(syn_type, config))
    }
}

#[cfg(test)]
mod test {
    use ligen::parser::Parser;
    use crate::types::type_::TypeParser;
    use crate::prelude::*;
    use super::*;

    // FIXME: Update this tests to use the mock module.

    fn test_pairs(input: Vec<(proc_macro2::TokenStream, Type)>) {
        let v: Vec<(Type, Type)> = input.into_iter().map(|(input, expected)| {
            (TypeParser::new().parse(input, &Default::default()).expect("Failed to parse syn::Type"), expected)
        }).collect();
        for (value, expected_value) in v {
            assert_eq!(value, expected_value);
        }
    }

    #[test]
    fn types_array() {
        test_pairs(vec![
            (quote! { [u8; 4] }, Type::array(Type::u8(), 4)),
            (quote! { [u8] }, Type::slice(Type::u8()))
        ]);
    }

    #[test]
    fn types_map() {
        test_pairs(vec![
            // (quote! { Vec<u8> }, Type::vector(Type::u8())),
        ]);
    }

    #[test]
    fn types_integer() {
        test_pairs(vec![
            (quote! { u8 }, Type::u8()),
            (quote! { u16 }, Type::u16()),
            (quote! { u32 }, Type::u32()),
            (quote! { u64 }, Type::u64()),
            (quote! { u128 }, Type::u128()),
            (quote! { usize }, Type::usize()),
            (quote! { i8 }, Type::i8()),
            (quote! { i16 }, Type::i16()),
            (quote! { i32 }, Type::i32()),
            (quote! { i64 }, Type::i64()),
            (quote! { i128 }, Type::i128()),
            (quote! { isize }, Type::isize()),
        ]);
    }

    #[test]
    fn types_float() {
        test_pairs(vec![
            (quote! { f32 }, Type::f32()),
            (quote! { f64 }, Type::f64()),
        ]);
    }

    #[test]
    fn types_boolean() -> Result<()> {
        assert_eq!(
            Type::boolean(),
            TypeParser::new().parse(quote! {bool}, &Default::default())?
        );
        Ok(())
    }

    #[test]
    fn types_character() -> Result<()> {
        assert_eq!(
            Type::character(),
            TypeParser::new().parse(quote! {char}, &Default::default())?
        );
        Ok(())
    }

    #[test]
    fn types_borrow_constant() -> Result<()> {
        assert_eq!(
            Type::constant_reference(Type::i32()),
            TypeParser::new().parse(quote! {&i32}, &Default::default())?
        );
        Ok(())
    }

    #[test]
    fn types_borrow_mutable() -> Result<()> {
        assert_eq!(
            Type::mutable_reference(Type::i32()),
            TypeParser::new().parse(quote! {&mut i32}, &Default::default())?
        );
        Ok(())
    }

    #[test]
    fn types_pointer_constant() -> Result<()> {
        assert_eq!(
            Type::constant_reference(Type::i32()),
            TypeParser::new().parse(quote! {*const i32}, &Default::default())?
        );
        Ok(())
    }

    #[test]
    fn types_pointer_mutable() -> Result<()> {
        assert_eq!(
            Type::mutable_reference(Type::i32()),
            TypeParser::new().parse(quote! {*mut i32}, &Default::default())?
        );
        Ok(())
    }
}

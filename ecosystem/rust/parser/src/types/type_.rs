use ligen::prelude::*;
use ligen::ir::Type;
use quote::ToTokens;
use syn::{TypeArray, TypeSlice};
use crate::{literal::LiteralParser, mutability::MutabilityParser};
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

impl Transformer<syn::Ident, Type> for TypeParser {
    fn transform(&self, input: syn::Ident, config: &Config) -> Result<Type> {
        Ok(PathParser::default().transform(input, config)?.into())
    }
}

impl Transformer<syn::Path, Type> for TypeParser {
    fn transform(&self, path: syn::Path, config: &Config) -> Result<Type> {
        let mut path = PathParser::default().transform(path, config)?;
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

impl Transformer<syn::Type, Type> for TypeParser {
    fn transform(&self, syn_type: syn::Type, config: &Config) -> Result<Type> {
        if let syn::Type::Path(syn::TypePath { path, .. }) = syn_type {
            Ok(self.transform(path, config)?)
        } else {
            match syn_type {
                syn::Type::Reference(syn::TypeReference { elem, mutability, .. }) |
                syn::Type::Ptr(syn::TypePtr { elem, mutability, .. }) => {
                    let mutability = self.mutability_parser.transform(mutability, config)?;
                    let type_ = TypeParser::new().transform(*elem, config)?;
                    Ok(Type::reference(mutability, type_))
                },
                syn::Type::Slice(TypeSlice { elem, .. }) => {
                    let type_ = TypeParser::new().transform(*elem, config)?;
                    Ok(Type::slice(type_))
                },
                syn::Type::Array(TypeArray { elem, len, .. }) => {
                    let len = self.literal_parser.transform(len, config)?;
                    let len = len.into_integer().map_err(|_| Error::Message("Array length literal isn't an integer.".into()))? as usize;
                    let type_ = TypeParser::new().transform(*elem, config)?;
                    Ok(Type::array(type_, len))
                },
                _ => Err(Error::Message(format!("\"{}\" not supported. Only Path, Reference and Ptr Types are currently supported", syn_type.to_token_stream()))),
            }
        }
    }
}

impl Transformer<proc_macro::TokenStream, Type> for TypeParser {
    fn transform(&self, input: proc_macro::TokenStream, config: &Config) -> Result<Type> {
        self.transform(proc_macro2::TokenStream::from(input), config)
    }
}

impl Transformer<proc_macro2::TokenStream, Type> for TypeParser {
    fn transform(&self, input: proc_macro2::TokenStream, config: &Config) -> Result<Type> {
        syn::parse2::<syn::Type>(input)
            .map_err(|e| Error::Message(format!("Failed to parse type: {}", e)))
            .and_then(|syn_type| self.transform(syn_type, config))
    }
}

#[cfg(test)]
mod test {
    use crate::types::type_::TypeParser;
    use crate::prelude::*;
    use super::*;

    // FIXME: Update this tests to use the mock module.

    fn test_pairs(input: Vec<(proc_macro2::TokenStream, Type)>) {
        let v: Vec<(Type, Type)> = input.into_iter().map(|(input, expected)| {
            (TypeParser::new().transform(input, &Default::default()).expect("Failed to parse syn::Type"), expected)
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
            TypeParser::new().transform(quote! {bool}, &Default::default())?
        );
        Ok(())
    }

    #[test]
    fn types_character() -> Result<()> {
        assert_eq!(
            Type::character(),
            TypeParser::new().transform(quote! {char}, &Default::default())?
        );
        Ok(())
    }

    #[test]
    fn types_borrow_constant() -> Result<()> {
        assert_eq!(
            Type::constant_reference(Type::i32()),
            TypeParser::new().transform(quote! {&i32}, &Default::default())?
        );
        Ok(())
    }

    #[test]
    fn types_borrow_mutable() -> Result<()> {
        assert_eq!(
            Type::mutable_reference(Type::i32()),
            TypeParser::new().transform(quote! {&mut i32}, &Default::default())?
        );
        Ok(())
    }

    #[test]
    fn types_pointer_constant() -> Result<()> {
        assert_eq!(
            Type::constant_reference(Type::i32()),
            TypeParser::new().transform(quote! {*const i32}, &Default::default())?
        );
        Ok(())
    }

    #[test]
    fn types_pointer_mutable() -> Result<()> {
        assert_eq!(
            Type::mutable_reference(Type::i32()),
            TypeParser::new().transform(quote! {*mut i32}, &Default::default())?
        );
        Ok(())
    }
}

use ligen::{ir::{Reference, Mutability, Type}, parsing::parser::ParserConfig};
use crate::prelude::*;
use ligen::parsing::parser::Parser;
use crate::path::PathParser;

pub struct TypeParser;

impl Parser<syn::Ident> for TypeParser {
    type Output = Type;
    fn parse(&self, input: syn::Ident, config: &ParserConfig) -> Result<Self::Output> {
        Ok(Type::Path(PathParser::default().parse(input, config)?))
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
        Ok(Type::Path(path))
    }
}

impl Parser<syn::Type> for TypeParser {
    type Output = Type;
    fn parse(&self, syn_type: syn::Type, config: &ParserConfig) -> Result<Self::Output> {
        if let syn::Type::Path(syn::TypePath { path, .. }) = syn_type {
            Ok(self.parse(path, config)?)
        } else {
            let reference = match &syn_type {
                syn::Type::Reference(syn::TypeReference {
                                         elem, mutability, ..
                                     }) => Some((elem, mutability)),
                syn::Type::Ptr(syn::TypePtr {
                                   elem, mutability, ..
                               }) => Some((elem, mutability)),
                _ => None,
            };
            if let Some((elem, mutability)) = reference {
                if let syn::Type::Path(syn::TypePath { path, .. }) = *elem.clone() {
                    let mutability = if mutability.is_none() { Mutability::Constant } else { Mutability::Mutable };
                    let type_ = Box::new(TypeParser.parse(path, config)?);
                    Ok(Self::Output::Reference(Reference { mutability, type_, }))
                } else {
                    Err(Error::Message("Couldn't find path".into()))
                }
            } else {
                Err(Error::Message("Only Path, Reference and Ptr Types are currently supported".into()))
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
    use ligen::parsing::parser::Parser;
    use crate::types::type_::TypeParser;
    use crate::prelude::*;
    use super::*;

    // FIXME: Update this tests to use the mock module.

    #[test]
    fn types_integer() {
        let vec: Vec<Type> = vec![
            quote! { u8 },
            quote! { u16 },
            quote! { u32 },
            quote! { u64 },
            quote! { u128 },
            quote! { usize },
            quote! { i8 },
            quote! { i16 },
            quote! { i32 },
            quote! { i64 },
            quote! { i128 },
            quote! { isize },
        ]
            .into_iter()
            .map(|x| {
                TypeParser.parse(x, &Default::default()).expect("Failed to convert from syn::Type")
            })
            .collect();
        let expected: Vec<Type> = vec![
            Type::u8(),
            Type::u16(),
            Type::u32(),
            Type::u64(),
            Type::u128(),
            Type::usize(),
            Type::i8(),
            Type::i16(),
            Type::i32(),
            Type::i64(),
            Type::i128(),
            Type::isize(),
        ]
            .into_iter()
            .collect();

        for (value, expected_value) in vec.iter().zip(expected.iter()) {
            assert_eq!(value, expected_value);
        }
    }

    #[test]
    fn types_float() {
        let vec: Vec<Type> = vec![quote! { f32 }, quote! { f64 }]
            .into_iter()
            .map(|x| {
                TypeParser.parse(x, &Default::default()).expect("Failed to convert from syn::Type")
            })
            .collect();
        let expected: Vec<Type> = vec![Type::f32(), Type::f64()]
            .into_iter()
            .collect();

        for (value, expected_value) in vec.iter().zip(expected.iter()) {
            assert_eq!(value, expected_value);
        }
    }

    #[test]
    fn types_boolean() -> Result<()> {
        assert_eq!(
            Type::boolean(),
            TypeParser.parse(quote! {bool}, &Default::default())?
        );
        Ok(())
    }

    #[test]
    fn types_character() -> Result<()> {
        assert_eq!(
            Type::character(),
            TypeParser.parse(quote! {char}, &Default::default())?
        );
        Ok(())
    }

    #[test]
    fn types_borrow_constant() -> Result<()> {
        assert_eq!(
            Type::Reference(
                Reference {
                    mutability: Mutability::Constant,
                    type_: Type::i32().into()
                }
            ),
            TypeParser.parse(quote! {&i32}, &Default::default())?
        );
        Ok(())
    }

    #[test]
    fn types_borrow_mutable() -> Result<()> {
        assert_eq!(
            Type::Reference(
                Reference {
                    mutability: Mutability::Mutable,
                    type_: Type::i32().into()
                }
            ),
            TypeParser.parse(quote! {&mut i32}, &Default::default())?
        );
        Ok(())
    }

    #[test]
    fn types_pointer_constant() -> Result<()> {
        assert_eq!(
            Type::Reference(Reference {
                mutability: Mutability::Constant,
                type_: Type::i32().into()
            }),
            TypeParser.parse(quote! {*const i32}, &Default::default())?
        );
        Ok(())
    }

    #[test]
    fn types_pointer_mutable() -> Result<()> {
        assert_eq!(
            Type::Reference(Reference {
                mutability: Mutability::Mutable,
                type_: Type::i32().into()
            }),
            TypeParser.parse(quote! {*mut i32}, &Default::default())?
        );
        Ok(())
    }
}

use ligen_ir::{Primitive, Reference, Mutability, Type};
use crate::prelude::*;
use ligen_parsing::Parser;
use crate::path::PathParser;
use crate::types::GenericsParser;
use crate::types::primitive::PrimitiveParser;

pub struct TypeParser;

impl Parser<syn::Path> for TypeParser {
    type Output = Type;
    fn parse(&self, path: syn::Path) -> Result<Self::Output> {
        if Primitive::is_primitive(PathParser.parse(path.clone())?) {
            Ok(Self::Output::Primitive(PrimitiveParser.parse(path)?))
        } else {
            let generics = path
                .segments
                .last()
                .map(|segment| GenericsParser.parse(segment.arguments.clone()).expect("Failed to parse generics."))
                .unwrap_or_default();
            Ok(Self::Output::Composite(PathParser.parse(path)?, generics))
        }
    }
}

impl Parser<syn::Type> for TypeParser {
    type Output = Type;
    fn parse(&self, syn_type: syn::Type) -> Result<Self::Output> {
        if let syn::Type::Path(syn::TypePath { path, .. }) = syn_type {
            Ok(self.parse(path)?)
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
                    let type_ = Box::new(TypeParser.parse(path)?);
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
    fn parse(&self, input: proc_macro::TokenStream) -> Result<Self::Output> {
        self.parse(proc_macro2::TokenStream::from(input))
    }
}

impl Parser<proc_macro2::TokenStream> for TypeParser {
    type Output = Type;
    fn parse(&self, input: proc_macro2::TokenStream) -> Result<Self::Output> {
        syn::parse2::<syn::Type>(input)
            .map_err(|e| Error::Message(format!("Failed to parse type: {}", e)))
            .and_then(|syn_type| self.parse(syn_type))
    }
}

impl ToTokens for Type {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Type::Primitive(primitive) => tokens.append_all(primitive.to_token_stream()),
            Type::Composite(composite, generics) => {
                tokens.append_all(composite.to_token_stream());
                tokens.append_all(generics.to_token_stream());
            },
            Type::Reference(reference) => tokens.append_all(reference.to_token_stream()),
        }
    }
}

#[cfg(test)]
mod test {
    use ligen_ir::{Float, Integer, Mutability};
    use ligen_parsing::Parser;
    use crate::types::type_::TypeParser;
    use crate::prelude::*;
    use super::*;

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
                TypeParser.parse(x).expect("Failed to convert from syn::Type")
            })
            .collect();
        let expected: Vec<Type> = vec![
            Integer::U8,
            Integer::U16,
            Integer::U32,
            Integer::U64,
            Integer::U128,
            Integer::USize,
            Integer::I8,
            Integer::I16,
            Integer::I32,
            Integer::I64,
            Integer::I128,
            Integer::ISize,
        ]
            .into_iter()
            .map(|x| Type::Primitive(Primitive::Integer(x)))
            .collect();

        let mut iter = vec.iter().zip(expected.iter());

        while let Some((value, expected_value)) = iter.next() {
            assert_eq!(value, expected_value);
        }
    }

    #[test]
    fn types_float() {
        let vec: Vec<Type> = vec![quote! { f32 }, quote! { f64 }]
            .into_iter()
            .map(|x| {
                TypeParser.parse(x).expect("Failed to convert from syn::Type")
            })
            .collect();
        let expected: Vec<Type> = vec![Float::F32, Float::F64]
            .into_iter()
            .map(|x| Type::Primitive(Primitive::Float(x)))
            .collect();

        let mut iter = vec.iter().zip(expected.iter());

        while let Some((value, expected_value)) = iter.next() {
            assert_eq!(value, expected_value);
        }
    }

    #[test]
    fn types_boolean() -> Result<()> {
        assert_eq!(
            Type::Primitive(Primitive::Boolean),
            TypeParser.parse(quote! {bool})?
        );
        Ok(())
    }

    #[test]
    fn types_character() -> Result<()> {
        assert_eq!(
            Type::Primitive(Primitive::Character),
            TypeParser.parse(quote! {char})?
        );
        Ok(())
    }

    #[test]
    fn types_borrow_constant() -> Result<()> {
        assert_eq!(
            Type::Reference(
                Reference {
                    mutability: Mutability::Constant,
                    type_: Box::new(
                        Type::Primitive(
                            Primitive::Integer(
                                Integer::I32
                            )
                        )
                    )
                }
            ),
            TypeParser.parse(quote! {&i32})?
        );
        Ok(())
    }

    #[test]
    fn types_borrow_mutable() -> Result<()> {
        assert_eq!(
            Type::Reference(
                Reference {
                    mutability: Mutability::Mutable,
                    type_: Box::new(
                        Type::Primitive(
                            Primitive::Integer(
                                Integer::I32
                            )
                        )
                    )
                }
            ),
            TypeParser.parse(quote! {&mut i32})?
        );
        Ok(())
    }

    #[test]
    fn types_pointer_constant() -> Result<()> {
        assert_eq!(
            Type::Reference(Reference {
                mutability: Mutability::Constant,
                type_: Box::new(
                    Type::Primitive(
                        Primitive::Integer(
                            Integer::I32
                        )
                    )
                )
            }),
            TypeParser.parse(quote! {*const i32})?
        );
        Ok(())
    }

    #[test]
    fn types_pointer_mutable() -> Result<()> {
        assert_eq!(
            Type::Reference(Reference {
                mutability: Mutability::Mutable,
                type_: Box::new(
                    Type::Primitive(
                        Primitive::Integer(
                            Integer::I32
                        )
                    )
                )
            }),
            TypeParser.parse(quote! {*mut i32})?
        );
        Ok(())
    }
}

//! Function parameter.

use crate::prelude::*;
use ligen_ir::{Identifier, Reference, Type, Mutability, Parameter};
use ligen_parsing::Parser;
use crate::identifier::IdentifierParser;
use crate::macro_attributes::attributes::AttributesParser;
use crate::types::TypeParser;

pub struct ParameterParser;

impl Parser<syn::FnArg> for ParameterParser {
    type Output = Parameter;

    fn parse(&self, fn_arg: syn::FnArg) -> Result<Self::Output> {
        match fn_arg {
            syn::FnArg::Typed(syn::PatType { pat, ty, attrs, .. }) => {
                if let syn::Pat::Ident(syn::PatIdent { ident, .. }) = *pat {
                    Ok(Self::Output {
                        attributes: AttributesParser.parse(attrs)?,
                        identifier: IdentifierParser.parse(ident)?,
                        type_: TypeParser.parse(*ty)?,
                    })
                } else {
                    Err(Error::Message("Identifier not found".into()))
                }
            }
            // TODO: Implement conversion for syn::Receiver. <- What does it mean?
            syn::FnArg::Receiver(syn::Receiver {
                                attrs,
                                reference,
                                mutability,
                                ..
                            }) => {
                let attributes = AttributesParser.parse(attrs)?;
                let identifier = Identifier::new("self").into();
                let type_ = reference
                    .map(|_| {
                        let mutability = if mutability.is_none() { Mutability::Constant } else { Mutability::Mutable };
                        let type_ = Box::new(Type::from(Identifier::new("Self")));
                        Type::Reference(Reference { mutability, type_ })
                    })
                    .unwrap_or_else(|| Type::from(Identifier::new("Self")));
                Ok(Self::Output { attributes, identifier, type_ })
            },
        }
    }
}

impl ToTokens for Parameter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = self.identifier.to_token_stream();
        let typ = self.type_.to_token_stream();
        tokens.append_all(quote! {#ident: #typ})
    }
}

#[cfg(test)]
mod test {
    use super::Parameter;
    use ligen_ir::{Primitive, Identifier, Integer, Reference, Type, Attribute, Mutability};
    use quote::quote;
    use syn::{parse_quote::parse};
    use ligen_parsing::Parser;
    use crate::function::parameter::ParameterParser;

    #[test]
    fn parameter_primitive() {
        assert_eq!(
            ParameterParser.parse(parse::<syn::FnArg>(quote! {
                #[attribute] integer: i32
            })).expect("Returned Error"),
            Parameter {
                attributes: Attribute::Group("attribute".into(), Default::default()).into(),
                identifier: Identifier::new("integer"),
                type_: Type::Primitive(Primitive::Integer(Integer::I32))
            }
        );
    }

    #[test]
    fn parameter_composite() {
        assert_eq!(
            ParameterParser.parse(parse::<syn::FnArg>(quote! {name: String})).expect("Returned Error"),
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("name"),
                type_: Type::Composite(Identifier::new("String").into(), Default::default())
            }
        );
    }

    #[test]
    fn parameter_borrow_constant() {
        assert_eq!(
            ParameterParser.parse(parse::<syn::FnArg>(quote! {name: &String})).expect("Returned Error"),
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("name"),
                type_: Type::Reference(
                    Reference {
                        mutability: Mutability::Constant,
                        type_: Box::new(Type::Composite(Identifier::new("String").into(), Default::default()))
                    }
                )

            }
        );
    }

    #[test]
    fn parameter_borrow_mutable() {
        assert_eq!(
            ParameterParser.parse(parse::<syn::FnArg>(quote! {name: &mut String}))
                .expect("Returned Error"),
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("name"),
                type_: Type::Reference(
                    Reference {
                        mutability: Mutability::Mutable,
                        type_: Box::new(Type::Composite(Identifier::new("String").into(), Default::default()))
                    }
                )

            }
        );
    }

    #[test]
    fn parameter_pointer_constant() {
        assert_eq!(
            ParameterParser.parse(parse::<syn::FnArg>(quote! {name: *const String}))
                .expect("Returned Error"),
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("name"),
                type_: Type::Reference(
                    Reference {
                        mutability: Mutability::Constant,
                        type_: Box::new(Type::Composite(Identifier::new("String").into(), Default::default()))
                    }
                )

            }
        );
    }

    #[test]
    fn parameter_pointer_mutable() {
        assert_eq!(
            ParameterParser.parse(parse::<syn::FnArg>(quote! {name: *mut String}))
                .expect("Returned Error"),
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("name"),
                type_: Type::Reference(
                    Reference {
                        mutability: Mutability::Mutable,
                        type_: Box::new(Type::Composite(Identifier::new("String").into(), Default::default()))
                    }
                )
            }
        );
    }

    #[test]
    fn parameter_receiver() {
        assert_eq!(
            ParameterParser.parse(parse::<syn::FnArg>(quote! {self})).expect("Returned Error"),
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("self").into(),
                type_: Type::Composite(Identifier::new("Self").into(), Default::default())
            }
        );
    }

    #[test]
    fn parameter_receiver_reference() {
        assert_eq!(
            ParameterParser.parse(parse::<syn::FnArg>(quote! {&self})).expect("Returned Error"),
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("self").into(),
                type_: Type::Reference(
                    Reference {
                        mutability: Mutability::Constant,
                        type_: Box::new(Type::Composite(Identifier::new("Self").into(), Default::default()))
                    }
                )
            }
        );
    }

    #[test]
    fn parameter_receiver_mutable() {
        assert_eq!(
            ParameterParser.parse(parse::<syn::FnArg>(quote! {&mut self})).expect("Returned Error"),
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("self").into(),
                type_: Type::Reference(
                    Reference {
                        mutability: Mutability::Mutable,
                        type_: Box::new(Type::Composite(Identifier::new("Self").into(), Default::default()))
                    }
                )
            }
        );
    }
}

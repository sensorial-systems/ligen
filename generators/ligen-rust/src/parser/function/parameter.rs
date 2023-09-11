//! Function parameter.

use crate::prelude::*;
use crate::{Identifier, Reference, Type, Mutability, Parameter};

impl TryFrom<SynFnArg> for Parameter {
    type Error = Error;

    fn try_from(SynFnArg(fn_arg): SynFnArg) -> Result<Self> {
        match fn_arg {
            syn::FnArg::Typed(syn::PatType { pat, ty, attrs, .. }) => {
                if let syn::Pat::Ident(syn::PatIdent { ident, .. }) = *pat {
                    Ok(Self {
                        attributes: LigenAttributes::try_from(attrs)?.into(),
                        identifier: SynIdent::from(ident).into(),
                        type_: Type::try_from(SynType::from(*ty)).expect("Failed to convert from Type"),
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
                let attributes = LigenAttributes::try_from(attrs)?.into();
                let identifier = Identifier::new("self").into();
                let type_ = reference
                    .map(|_| {
                        let mutability = if mutability.is_none() { Mutability::Constant } else { Mutability::Mutable };
                        let type_ = Box::new(Type::from(Identifier::new("Self")));
                        Type::Reference(Reference { mutability, type_ })
                    })
                    .unwrap_or_else(|| Type::from(Identifier::new("Self")));
                Ok(Self { attributes, identifier, type_ })
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

    use std::convert::TryFrom;

    use super::Parameter;
    use crate::{Primitive, Identifier, Integer, Reference, Type, Attribute, Mutability};
    use quote::quote;
    use syn::{parse_quote::parse};
    use crate::prelude::SynFnArg;

    #[test]
    fn parameter_primitive() {
        assert_eq!(
            Parameter::try_from(SynFnArg(parse::<syn::FnArg>(quote! {
                #[attribute] integer: i32
            }))).expect("Returned Error"),
            Parameter {
                attributes: Attribute::Group("attribute".into(), Default::default()).into(),
                identifier: Identifier::new("integer"),
                type_: Type::Primitive(Primitive::Integer(Integer::I32))
            }
        );
    }

    #[test]
    fn parameter_compound() {
        assert_eq!(
            Parameter::try_from(SynFnArg(parse::<syn::FnArg>(quote! {name: String}))).expect("Returned Error"),
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("name"),
                type_: Type::Compound(Identifier::new("String").into(), Default::default())
            }
        );
    }

    #[test]
    fn parameter_borrow_constant() {
        assert_eq!(
            Parameter::try_from(SynFnArg(parse::<syn::FnArg>(quote! {name: &String}))).expect("Returned Error"),
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("name"),
                type_: Type::Reference(
                    Reference {
                        mutability: Mutability::Constant,
                        type_: Box::new(Type::Compound(Identifier::new("String").into(), Default::default()))
                    }
                )

            }
        );
    }

    #[test]
    fn parameter_borrow_mutable() {
        assert_eq!(
            Parameter::try_from(SynFnArg(parse::<syn::FnArg>(quote! {name: &mut String})))
                .expect("Returned Error"),
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("name"),
                type_: Type::Reference(
                    Reference {
                        mutability: Mutability::Mutable,
                        type_: Box::new(Type::Compound(Identifier::new("String").into(), Default::default()))
                    }
                )

            }
        );
    }

    #[test]
    fn parameter_pointer_constant() {
        assert_eq!(
            Parameter::try_from(SynFnArg(parse::<syn::FnArg>(quote! {name: *const String})))
                .expect("Returned Error"),
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("name"),
                type_: Type::Reference(
                    Reference {
                        mutability: Mutability::Constant,
                        type_: Box::new(Type::Compound(Identifier::new("String").into(), Default::default()))
                    }
                )

            }
        );
    }

    #[test]
    fn parameter_pointer_mutable() {
        assert_eq!(
            Parameter::try_from(SynFnArg(parse::<syn::FnArg>(quote! {name: *mut String})))
                .expect("Returned Error"),
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("name"),
                type_: Type::Reference(
                    Reference {
                        mutability: Mutability::Mutable,
                        type_: Box::new(Type::Compound(Identifier::new("String").into(), Default::default()))
                    }
                )
            }
        );
    }

    #[test]
    fn parameter_receiver() {
        assert_eq!(
            Parameter::try_from(SynFnArg(parse::<syn::FnArg>(quote! {self}))).expect("Returned Error"),
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("self").into(),
                type_: Type::Compound(Identifier::new("Self").into(), Default::default())
            }
        );
    }

    #[test]
    fn parameter_receiver_reference() {
        assert_eq!(
            Parameter::try_from(SynFnArg(parse::<syn::FnArg>(quote! {&self}))).expect("Returned Error"),
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("self").into(),
                type_: Type::Reference(
                    Reference {
                        mutability: Mutability::Constant,
                        type_: Box::new(Type::Compound(Identifier::new("Self").into(), Default::default()))
                    }
                )
            }
        );
    }

    #[test]
    fn parameter_receiver_mutable() {
        assert_eq!(
            Parameter::try_from(SynFnArg(parse::<syn::FnArg>(quote! {&mut self}))).expect("Returned Error"),
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("self").into(),
                type_: Type::Reference(
                    Reference {
                        mutability: Mutability::Mutable,
                        type_: Box::new(Type::Compound(Identifier::new("Self").into(), Default::default()))
                    }
                )
            }
        );
    }
}

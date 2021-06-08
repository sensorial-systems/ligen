use crate::ir::{Identifier, Reference, Type, ReferenceKind};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use std::convert::TryFrom;
use syn::FnArg;

#[derive(Debug, PartialEq, Clone)]
/// Parameter Struct
pub struct Parameter {
    /// identifier field
    pub identifier: Identifier,
    /// type_ field
    pub type_: Type,
}

impl TryFrom<FnArg> for Parameter {
    type Error = &'static str;

    fn try_from(fn_arg: FnArg) -> Result<Self, Self::Error> {
        match fn_arg {
            FnArg::Typed(syn::PatType { pat, ty, .. }) => {
                if let syn::Pat::Ident(syn::PatIdent { ident, .. }) = *pat {
                    Ok(Self {
                        identifier: ident.into(),
                        type_: Type::try_from(*ty).expect("Failed to convert from Type"),
                    })
                } else {
                    Err("Identifier not found")
                }
            }
            FnArg::Receiver(syn::Receiver {
                reference,
                mutability,
                ..
            }) => {
                let identifier = Identifier::new("self");
                let type_ = reference
                    .map(|_| {
                        let kind = ReferenceKind::Borrow;
                        let is_constant = mutability.is_none();
                        let type_ = Box::new(Type::Compound(Identifier::new("Self")));
                        Type::Reference(Reference { kind, is_constant, type_ })
                    })
                    .unwrap_or_else(|| Type::Compound(Identifier::new("Self")));
                Ok(Self { identifier, type_ })
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
    use crate::ir::{Atomic, Identifier, Integer, Reference, Type, ReferenceKind};
    use quote::quote;
    use syn::{parse_quote::parse, FnArg};

    #[test]
    fn parameter_atomic() {
        assert_eq!(
            Parameter::try_from(parse::<FnArg>(quote! {integer: i32})).expect("Returned Error"),
            Parameter {
                identifier: Identifier::new("integer"),
                type_: Type::Atomic(Atomic::Integer(Integer::I32))
            }
        );
    }

    #[test]
    fn parameter_compound() {
        assert_eq!(
            Parameter::try_from(parse::<FnArg>(quote! {name: String})).expect("Returned Error"),
            Parameter {
                identifier: Identifier::new("name"),
                type_: Type::Compound(Identifier::new("String"))
            }
        );
    }

    #[test]
    fn parameter_borrow_constant() {
        assert_eq!(
            Parameter::try_from(parse::<FnArg>(quote! {name: &String})).expect("Returned Error"),
            Parameter {
                identifier: Identifier::new("name"),
                type_: Type::Reference(
                    Reference {
                        kind: ReferenceKind::Borrow,
                        is_constant: true,
                        type_: Box::new(Type::Compound(Identifier::new("String")))
                    }
                )

            }
        );
    }

    #[test]
    fn parameter_borrow_mutable() {
        assert_eq!(
            Parameter::try_from(parse::<FnArg>(quote! {name: &mut String}))
                .expect("Returned Error"),
            Parameter {
                identifier: Identifier::new("name"),
                type_: Type::Reference(
                    Reference {
                        kind: ReferenceKind::Borrow,
                        is_constant: false,
                        type_: Box::new(Type::Compound(Identifier::new("String")))
                    }
                )

            }
        );
    }

    #[test]
    fn parameter_pointer_constant() {
        assert_eq!(
            Parameter::try_from(parse::<FnArg>(quote! {name: *const String}))
                .expect("Returned Error"),
            Parameter {
                identifier: Identifier::new("name"),
                type_: Type::Reference(
                    Reference {
                        kind: ReferenceKind::Pointer,
                        is_constant: true,
                        type_: Box::new(Type::Compound(Identifier::new("String")))
                    }
                )

            }
        );
    }

    #[test]
    fn parameter_pointer_mutable() {
        assert_eq!(
            Parameter::try_from(parse::<FnArg>(quote! {name: *mut String}))
                .expect("Returned Error"),
            Parameter {
                identifier: Identifier::new("name"),
                type_: Type::Reference(
                    Reference {
                        kind: ReferenceKind::Pointer,
                        is_constant: false,
                        type_: Box::new(Type::Compound(Identifier::new("String")))
                    }
                )
            }
        );
    }

    #[test]
    fn parameter_receiver() {
        assert_eq!(
            Parameter::try_from(parse::<FnArg>(quote! {self})).expect("Returned Error"),
            Parameter {
                identifier: Identifier::new("self"),
                type_: Type::Compound(Identifier::new("Self"))
            }
        );
    }

    #[test]
    fn parameter_receiver_reference() {
        assert_eq!(
            Parameter::try_from(parse::<FnArg>(quote! {&self})).expect("Returned Error"),
            Parameter {
                identifier: Identifier::new("self"),
                type_: Type::Reference(
                    Reference {
                        kind: ReferenceKind::Borrow,
                        is_constant: true,
                        type_: Box::new(Type::Compound(Identifier::new("Self")))
                    }
                )
            }
        );
    }

    #[test]
    fn parameter_receiver_mutable() {
        assert_eq!(
            Parameter::try_from(parse::<FnArg>(quote! {&mut self})).expect("Returned Error"),
            Parameter {
                identifier: Identifier::new("self"),
                type_: Type::Reference(
                    Reference {
                        kind: ReferenceKind::Borrow,
                        is_constant: false,
                        type_: Box::new(Type::Compound(Identifier::new("Self")))
                    }
                )
            }
        );
    }
}

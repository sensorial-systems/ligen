use crate::ir::{Borrow, Identifier, Reference, Type};
use std::convert::TryFrom;
use syn::FnArg;

#[derive(Debug, PartialEq)]
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
            }) => Ok(Self {
                identifier: Identifier {
                    name: String::from("self"),
                },

                type_: match (reference, mutability) {
                    (Some(_x), Some(_y)) => Type::Reference(Reference::Borrow(Borrow::Mutable(
                        Box::new(Type::Compound(Identifier {
                            name: String::from("Self"),
                        })),
                    ))),
                    (Some(_x), None) => Type::Reference(Reference::Borrow(Borrow::Constant(
                        Box::new(Type::Compound(Identifier {
                            name: String::from("Self"),
                        })),
                    ))),
                    (None, None) => Type::Compound(Identifier {
                        name: String::from("Self"),
                    }),
                    (None, Some(_y)) => panic!("Non-Reference Mutable Self"),
                },
            }),
        }
    }
}
#[cfg(test)]
mod test {

    use std::convert::TryFrom;

    use super::Parameter;
    use crate::ir::{Atomic, Borrow, Identifier, Integer, Pointer, Reference, Type};
    use quote::quote;
    use syn::{parse_quote::parse, FnArg};

    #[test]
    fn parameter_atomic() {
        assert_eq!(
            Parameter::try_from(parse::<FnArg>(quote! {integer: i32})).expect("Returned Error"),
            Parameter {
                identifier: Identifier {
                    name: String::from("integer")
                },
                type_: Type::Atomic(Atomic::Integer(Integer::I32))
            }
        );
    }

    #[test]
    fn parameter_compound() {
        assert_eq!(
            Parameter::try_from(parse::<FnArg>(quote! {name: String})).expect("Returned Error"),
            Parameter {
                identifier: Identifier {
                    name: String::from("name")
                },
                type_: Type::Compound(Identifier {
                    name: String::from("String")
                })
            }
        );
    }

    #[test]
    fn parameter_borrow_constant() {
        assert_eq!(
            Parameter::try_from(parse::<FnArg>(quote! {name: &String})).expect("Returned Error"),
            Parameter {
                identifier: Identifier {
                    name: String::from("name")
                },
                type_: Type::Reference(Reference::Borrow(Borrow::Constant(Box::new(
                    Type::Compound(Identifier {
                        name: String::from("String")
                    })
                ))))
            }
        );
    }

    #[test]
    fn parameter_borrow_mutable() {
        assert_eq!(
            Parameter::try_from(parse::<FnArg>(quote! {name: &mut String}))
                .expect("Returned Error"),
            Parameter {
                identifier: Identifier {
                    name: String::from("name")
                },
                type_: Type::Reference(Reference::Borrow(Borrow::Mutable(Box::new(
                    Type::Compound(Identifier {
                        name: String::from("String")
                    })
                ))))
            }
        );
    }

    #[test]
    fn parameter_pointer_constant() {
        assert_eq!(
            Parameter::try_from(parse::<FnArg>(quote! {name: *const String}))
                .expect("Returned Error"),
            Parameter {
                identifier: Identifier {
                    name: String::from("name")
                },
                type_: Type::Reference(Reference::Pointer(Pointer::Constant(Box::new(
                    Type::Compound(Identifier {
                        name: String::from("String")
                    })
                ))))
            }
        );
    }

    #[test]
    fn parameter_pointer_mutable() {
        assert_eq!(
            Parameter::try_from(parse::<FnArg>(quote! {name: *mut String}))
                .expect("Returned Error"),
            Parameter {
                identifier: Identifier {
                    name: String::from("name")
                },
                type_: Type::Reference(Reference::Pointer(Pointer::Mutable(Box::new(
                    Type::Compound(Identifier {
                        name: String::from("String")
                    })
                ))))
            }
        );
    }

    #[test]
    fn parameter_receiver() {
        assert_eq!(
            Parameter::try_from(parse::<FnArg>(quote! {self})).expect("Returned Error"),
            Parameter {
                identifier: Identifier {
                    name: String::from("self")
                },
                type_: Type::Compound(Identifier {
                    name: String::from("Self")
                })
            }
        );
    }

    #[test]
    fn parameter_receiver_reference() {
        assert_eq!(
            Parameter::try_from(parse::<FnArg>(quote! {&self})).expect("Returned Error"),
            Parameter {
                identifier: Identifier {
                    name: String::from("self")
                },
                type_: Type::Reference(Reference::Borrow(Borrow::Constant(Box::new(
                    Type::Compound(Identifier {
                        name: String::from("Self")
                    })
                ))))
            }
        );
    }

    #[test]
    fn parameter_receiver_mutable() {
        assert_eq!(
            Parameter::try_from(parse::<FnArg>(quote! {&mut self})).expect("Returned Error"),
            Parameter {
                identifier: Identifier {
                    name: String::from("self")
                },
                type_: Type::Reference(Reference::Borrow(Borrow::Mutable(Box::new(
                    Type::Compound(Identifier {
                        name: String::from("Self")
                    })
                ))))
            }
        );
    }
}

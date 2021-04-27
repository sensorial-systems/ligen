use crate::ir::{Borrow, Identifier, Reference, Type};
use syn::FnArg;

#[derive(Debug, PartialEq)]
/// Argument Struct
pub struct Argument {
    pub identifier: Identifier,
    pub type_: Type,
}

impl From<FnArg> for Argument {
    fn from(fn_arg: FnArg) -> Self {
        match fn_arg {
            FnArg::Typed(syn::PatType { pat, ty, .. }) => {
                if let syn::Pat::Ident(syn::PatIdent { ident, .. }) = *pat {
                    Self {
                        identifier: Identifier::from(ident),
                        type_: Type::from(*ty),
                    }
                } else {
                    panic!("Identifier not found");
                }
            }
            FnArg::Receiver(syn::Receiver {
                reference,
                mutability,
                ..
            }) => Self {
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
            },
        }
    }
}
#[cfg(test)]
mod test {

    use super::Argument;
    use crate::ir::{Atomic, Borrow, Identifier, Integer, Pointer, Reference, Type};
    use quote::quote;
    use syn::{parse_quote::parse, FnArg};

    #[test]
    fn argument_atomic() {
        assert_eq!(
            Argument::from(parse::<FnArg>(quote! {integer: i32})),
            Argument {
                identifier: Identifier {
                    name: String::from("integer")
                },
                type_: Type::Atomic(Atomic::Integer(Integer::I32))
            }
        );
    }

    #[test]
    fn argument_compound() {
        assert_eq!(
            Argument::from(parse::<FnArg>(quote! {name: String})),
            Argument {
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
    fn argument_borrow_constant() {
        assert_eq!(
            Argument::from(parse::<FnArg>(quote! {name: &String})),
            Argument {
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
    fn argument_borrow_mutable() {
        assert_eq!(
            Argument::from(parse::<FnArg>(quote! {name: &mut String})),
            Argument {
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
    fn argument_pointer_constant() {
        assert_eq!(
            Argument::from(parse::<FnArg>(quote! {name: *const String})),
            Argument {
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
    fn argument_pointer_mutable() {
        assert_eq!(
            Argument::from(parse::<FnArg>(quote! {name: *mut String})),
            Argument {
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
    fn argument_receiver() {
        assert_eq!(
            Argument::from(parse::<FnArg>(quote! {self})),
            Argument {
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
    fn argument_receiver_reference() {
        assert_eq!(
            Argument::from(parse::<FnArg>(quote! {&self})),
            Argument {
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
    fn argument_receiver_mutable() {
        assert_eq!(
            Argument::from(parse::<FnArg>(quote! {&mut self})),
            Argument {
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

use crate::ir::{Argument, Attribute, Attributes, Identifier, Type};
use syn::ItemFn;

#[derive(Debug, PartialEq, Copy, Clone)]
/// Async Struct
pub struct Async;

#[derive(Debug, PartialEq)]
/// Function Struct
pub struct Function {
    pub attributes: Attributes,
    pub asyncness: Option<Async>,
    pub identifier: Identifier,
    pub input: Vec<Argument>,
    pub output: Option<Type>,
}

impl From<ItemFn> for Function {
    fn from(item_fn: ItemFn) -> Self {
        let syn::Signature {
            asyncness,
            ident,
            inputs,
            output,
            ..
        } = item_fn.sig;
        let input: Vec<Argument> = inputs
            .clone()
            .into_iter()
            .map(|x| Argument::from(x))
            .collect();
        let output: Option<Type> = match output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_x, y) => Some(Type::from(*y)),
        };
        Self {
            attributes: Attributes {
                attributes: item_fn
                    .attrs
                    .into_iter()
                    .map(|x| Attribute::from(x.parse_meta().expect("Failed to parse Meta")))
                    .collect(),
            },
            asyncness: match asyncness {
                Some(_x) => Some(Async),
                None => None,
            },
            identifier: Identifier::from(ident),
            input,
            output,
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Async, Function, ItemFn, Type};
    use crate::ir::{Argument, Attribute, Attributes, Borrow, Identifier, Literal, Reference};
    use quote::quote;
    use syn::parse_quote::parse;

    #[test]
    fn function() {
        assert_eq!(
            Function::from(parse::<ItemFn>(quote! {fn test() {}})),
            Function {
                attributes: Attributes { attributes: vec![] },
                asyncness: None,
                identifier: Identifier {
                    name: String::from("test")
                },
                input: vec![],
                output: None
            }
        );
    }

    #[test]
    fn function_input() {
        assert_eq!(
            Function::from(parse::<ItemFn>(quote! {fn test(a: String, b: String) {}})),
            Function {
                attributes: Attributes { attributes: vec![] },
                asyncness: None,
                identifier: Identifier {
                    name: String::from("test")
                },
                input: vec![
                    Argument {
                        identifier: Identifier {
                            name: String::from("a")
                        },
                        type_: Type::Compound(Identifier {
                            name: String::from("String")
                        })
                    },
                    Argument {
                        identifier: Identifier {
                            name: String::from("b")
                        },
                        type_: Type::Compound(Identifier {
                            name: String::from("String")
                        })
                    },
                ],
                output: None
            }
        );
    }

    #[test]
    fn function_output() {
        assert_eq!(
            Function::from(parse::<ItemFn>(quote! {fn test() -> String {}})),
            Function {
                attributes: Attributes { attributes: vec![] },
                asyncness: None,
                identifier: Identifier {
                    name: String::from("test")
                },
                input: vec![],
                output: Some(Type::Compound(Identifier {
                    name: String::from("String")
                }))
            }
        );
    }

    #[test]
    fn function_input_output() {
        assert_eq!(
            Function::from(parse::<ItemFn>(
                quote! {fn test(a: String, b: &String, c: &mut String) -> &String {}}
            )),
            Function {
                attributes: Attributes { attributes: vec![] },
                asyncness: None,
                identifier: Identifier {
                    name: String::from("test")
                },
                input: vec![
                    Argument {
                        identifier: Identifier {
                            name: String::from("a")
                        },
                        type_: Type::Compound(Identifier {
                            name: String::from("String")
                        })
                    },
                    Argument {
                        identifier: Identifier {
                            name: String::from("b")
                        },
                        type_: Type::Reference(Reference::Borrow(Borrow::Constant(Box::new(
                            Type::Compound(Identifier {
                                name: String::from("String")
                            })
                        ))))
                    },
                    Argument {
                        identifier: Identifier {
                            name: String::from("c")
                        },
                        type_: Type::Reference(Reference::Borrow(Borrow::Mutable(Box::new(
                            Type::Compound(Identifier {
                                name: String::from("String")
                            })
                        ))))
                    },
                ],
                output: Some(Type::Reference(Reference::Borrow(Borrow::Constant(
                    Box::new(Type::Compound(Identifier {
                        name: String::from("String")
                    }))
                ))))
            }
        );
    }

    #[test]
    fn function_attribute() {
        assert_eq!(
            Function::from(parse::<ItemFn>(quote! {
                #[test(a = "b")]
                fn test() {}
            })),
            Function {
                attributes: Attributes {
                    attributes: vec![Attribute::Group(
                        Identifier::new("test"),
                        Attributes {
                            attributes: vec![Attribute::Named(
                                Identifier::new("a"),
                                Literal::String(String::from("b"))
                            )]
                        }
                    )]
                },
                asyncness: None,
                identifier: Identifier {
                    name: String::from("test")
                },
                input: vec![],
                output: None
            }
        );
    }

    #[test]
    fn function_async() {
        assert_eq!(
            Function::from(parse::<ItemFn>(quote! {async fn test() {}})),
            Function {
                attributes: Attributes { attributes: vec![] },
                asyncness: Some(Async),
                identifier: Identifier {
                    name: String::from("test")
                },
                input: vec![],
                output: None
            }
        );
    }

    #[test]
    fn function_complete() {
        assert_eq!(
            Function::from(parse::<ItemFn>(quote! {
            #[test(a = "b")]
                async fn test(a: String, b: &String, c: &mut String) -> &String {}
            })),
            Function {
                attributes: Attributes {
                    attributes: vec![Attribute::Group(
                        Identifier::new("test"),
                        Attributes {
                            attributes: vec![Attribute::Named(
                                Identifier::new("a"),
                                Literal::String(String::from("b"))
                            )]
                        }
                    )]
                },
                asyncness: Some(Async),
                identifier: Identifier {
                    name: String::from("test")
                },
                input: vec![
                    Argument {
                        identifier: Identifier {
                            name: String::from("a")
                        },
                        type_: Type::Compound(Identifier {
                            name: String::from("String")
                        })
                    },
                    Argument {
                        identifier: Identifier {
                            name: String::from("b")
                        },
                        type_: Type::Reference(Reference::Borrow(Borrow::Constant(Box::new(
                            Type::Compound(Identifier {
                                name: String::from("String")
                            })
                        ))))
                    },
                    Argument {
                        identifier: Identifier {
                            name: String::from("c")
                        },
                        type_: Type::Reference(Reference::Borrow(Borrow::Mutable(Box::new(
                            Type::Compound(Identifier {
                                name: String::from("String")
                            })
                        ))))
                    },
                ],
                output: Some(Type::Reference(Reference::Borrow(Borrow::Constant(
                    Box::new(Type::Compound(Identifier {
                        name: String::from("String")
                    }))
                ))))
            }
        );
    }
}
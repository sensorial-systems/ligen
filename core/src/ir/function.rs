use crate::ir::{Attributes, Identifier, Parameter, Type};
use std::convert::{TryFrom, TryInto};
use syn::{ImplItemMethod, ItemFn};

#[derive(Debug, PartialEq, Copy, Clone)]
/// Async Struct
pub struct Async;

#[derive(Debug, PartialEq)]
/// Function Struct
pub struct Function {
    /// attributes field
    pub attributes: Attributes,
    /// asyncness field
    pub asyncness: Option<Async>,
    /// identifier field
    pub identifier: Identifier,
    /// input field
    pub input: Vec<Parameter>,
    /// output field
    pub output: Option<Type>,
}

macro_rules! impl_function {
    ($T:ident) => {
        impl From<$T> for Function {
            fn from(item_fn: $T) -> Self {
                let syn::Signature {
                    asyncness,
                    ident,
                    inputs,
                    output,
                    ..
                } = item_fn.sig;
                let input: Vec<Parameter> = inputs
                    .clone()
                    .into_iter()
                    .map(|x| x.try_into().expect("Failed to convert Parameter"))
                    .collect();
                let output: Option<Type> = match output {
                    syn::ReturnType::Default => None,
                    syn::ReturnType::Type(_x, y) => {
                        Some(Type::try_from(*y).expect("Failed to convert from ReturnType::Type"))
                    }
                };
                Self {
                    attributes: Attributes {
                        attributes: item_fn
                            .attrs
                            .into_iter()
                            .map(|x| x.parse_meta().expect("Failed to parse Meta").into())
                            .collect(),
                    },
                    asyncness: match asyncness {
                        Some(_x) => Some(Async),
                        None => None,
                    },
                    identifier: ident.into(),
                    input,
                    output,
                }
            }
        }
    };
}

impl_function!(ItemFn);
impl_function!(ImplItemMethod);

#[cfg(test)]
mod test {
    use super::{Async, Function, ImplItemMethod, ItemFn, Type};
    use crate::ir::{Attribute, Attributes, Borrow, Identifier, Literal, Parameter, Reference};
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
    fn function_impl() {
        assert_eq!(
            Function::from(parse::<ImplItemMethod>(quote! {fn test() {}})),
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
                    Parameter {
                        identifier: Identifier {
                            name: String::from("a")
                        },
                        type_: Type::Compound(Identifier {
                            name: String::from("String")
                        })
                    },
                    Parameter {
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
                    Parameter {
                        identifier: Identifier {
                            name: String::from("a")
                        },
                        type_: Type::Compound(Identifier {
                            name: String::from("String")
                        })
                    },
                    Parameter {
                        identifier: Identifier {
                            name: String::from("b")
                        },
                        type_: Type::Reference(Reference::Borrow(Borrow::Constant(Box::new(
                            Type::Compound(Identifier {
                                name: String::from("String")
                            })
                        ))))
                    },
                    Parameter {
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
                    Parameter {
                        identifier: Identifier {
                            name: String::from("a")
                        },
                        type_: Type::Compound(Identifier {
                            name: String::from("String")
                        })
                    },
                    Parameter {
                        identifier: Identifier {
                            name: String::from("b")
                        },
                        type_: Type::Reference(Reference::Borrow(Borrow::Constant(Box::new(
                            Type::Compound(Identifier {
                                name: String::from("String")
                            })
                        ))))
                    },
                    Parameter {
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

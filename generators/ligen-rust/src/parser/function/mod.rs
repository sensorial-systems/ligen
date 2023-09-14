use ligen_ir::Identifier;
use crate::prelude::*;

use crate::{Synchrony, Attributes, Function, Parameter, Type, Visibility};

pub mod parameter;
pub mod method;

impl From<SynItemFn> for Function {
    fn from(SynItemFn(item_fn): SynItemFn) -> Self {
        let syn::Signature {
            asyncness,
            ident,
            inputs,
            output,
            ..
        } = item_fn.sig;
        let inputs: Vec<Parameter> = inputs
            .clone()
            .into_iter()
            .map(|x| SynFnArg::from(x).try_into().expect("Failed to convert Parameter"))
            .collect();
        let output: Option<Type> = match output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_x, y) => {
                Some(Type::try_from(SynType::from(*y)).expect("Failed to convert from ReturnType::Type"))
            }
        };
        Self {
            attributes: Attributes {
                attributes: item_fn
                    .attrs
                    .into_iter()
                    .map(|x| SynMeta::from(x.parse_meta().expect("Failed to parse Meta")).into())
                    .collect(),
            },
            visibility: Visibility::from(SynVisibility::from(item_fn.vis)),
            synchrony: match asyncness {
                Some(_x) => Synchrony::Asynchronous,
                None => Synchrony::Synchronous,
            },
            path: Identifier::from(SynIdent::from(ident)).into(),
            inputs,
            output,
        }
    }
}

#[cfg(test)]
mod test {
    use quote::quote;
    use syn::parse_quote::parse;
    use ligen_ir::Synchrony;

    use crate::{Attribute, Attributes, Identifier, Literal, Mutability, Parameter, Reference, Visibility};
    use crate::prelude::SynItemFn;

    use super::{Function, Type};

    #[test]
    fn function() {
        assert_eq!(
            Function::from(SynItemFn(parse::<syn::ItemFn>(quote! {fn test() {}}))),
            Function {
                attributes: Attributes { attributes: vec![] },
                visibility: Visibility::Private,
                synchrony: Synchrony::Synchronous,
                path: Identifier::new("test").into(),
                inputs: vec![],
                output: None
            }
        );
    }

    #[test]
    fn function_impl() {
        assert_eq!(
            Function::from(SynItemFn(parse::<syn::ItemFn>(quote! {fn test() {}}))),
            Function {
                attributes: Attributes { attributes: vec![] },
                visibility: Visibility::Private,
                synchrony: Synchrony::Synchronous,
                path: Identifier::new("test").into(),
                inputs: vec![],
                output: None
            }
        );
    }

    #[test]
    fn function_input() {
        assert_eq!(
            Function::from(SynItemFn(parse::<syn::ItemFn>(quote! {fn test(a: String, b: String) {}}))),
            Function {
                attributes: Attributes { attributes: vec![] },
                visibility: Visibility::Private,
                synchrony: Synchrony::Synchronous,
                path: Identifier::new("test").into(),
                inputs: vec![
                    Parameter {
                        attributes: Default::default(),
                        identifier: Identifier::new("a"),
                        type_: Type::Composite(Identifier::new("String").into(), Default::default())
                    },
                    Parameter {
                        attributes: Default::default(),
                        identifier: Identifier::new("b"),
                        type_: Type::Composite(Identifier::new("String").into(), Default::default())
                    },
                ],
                output: None
            }
        );
    }

    #[test]
    fn function_output() {
        assert_eq!(
            Function::from(SynItemFn(parse::<syn::ItemFn>(quote! {fn test() -> String {}}))),
            Function {
                attributes: Attributes { attributes: vec![] },
                visibility: Visibility::Private,
                synchrony: Synchrony::Synchronous,
                path: Identifier::new("test").into(),
                inputs: vec![],
                output: Some(Type::Composite(Identifier::new("String").into(), Default::default()))
            }
        );
    }

    #[test]
    fn function_input_output() {
        assert_eq!(
            Function::from(SynItemFn(parse::<syn::ItemFn>(
                quote! {fn test(a: String, b: &String, c: &mut String) -> &String {}}
            ))),
            Function {
                attributes: Attributes { attributes: vec![] },
                visibility: Visibility::Private,
                synchrony: Synchrony::Synchronous,
                path: Identifier::new("test").into(),
                inputs: vec![
                    Parameter {
                        attributes: Default::default(),
                        identifier: Identifier::new("a"),
                        type_: Type::Composite(Identifier::new("String").into(), Default::default())
                    },
                    Parameter {
                        attributes: Default::default(),
                        identifier: Identifier::new("b"),
                        type_: Type::Reference(Reference {
                            mutability: Mutability::Constant,
                            type_: Box::new(Type::Composite(Identifier::new("String").into(), Default::default()))
                        })
                    },
                    Parameter {
                        attributes: Default::default(),
                        identifier: Identifier::new("c"),
                        type_: Type::Reference(Reference {
                            mutability: Mutability::Mutable,
                            type_: Box::new(Type::Composite(Identifier::new("String").into(), Default::default()))
                        })
                    },
                ],
                output: Some(Type::Reference(Reference {
                    mutability: Mutability::Constant,
                    type_: Box::new(Type::Composite(Identifier::new("String").into(), Default::default()))
                }))
            }
        );
    }

    #[test]
    fn function_attribute() {
        assert_eq!(
            Function::from(SynItemFn(parse::<syn::ItemFn>(quote! {
                #[test(a = "b")]
                fn test() {}
            }))),
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
                visibility: Visibility::Private,
                synchrony: Synchrony::Synchronous,
                path: Identifier::new("test").into(),
                inputs: vec![],
                output: None
            }
        );
    }

    #[test]
    fn function_async() {
        assert_eq!(
            Function::from(SynItemFn(parse::<syn::ItemFn>(quote! {async fn test() {}}))),
            Function {
                attributes: Attributes { attributes: vec![] },
                visibility: Visibility::Private,
                synchrony: Synchrony::Asynchronous,
                path: Identifier::new("test").into(),
                inputs: vec![],
                output: None
            }
        );
    }

    #[test]
    fn function_complete() {
        assert_eq!(
            Function::from(SynItemFn(parse::<syn::ItemFn>(quote! {
            #[test(a = "b")]
                async fn test(a: String, b: &String, c: &mut String) -> &String {}
            }))),
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
                visibility: Visibility::Private,
                synchrony: Synchrony::Asynchronous,
                path: Identifier::new("test").into(),
                inputs: vec![
                    Parameter {
                        attributes: Default::default(),
                        identifier: Identifier::new("a"),
                        type_: Type::Composite(Identifier::new("String").into(), Default::default())
                    },
                    Parameter {
                        attributes: Default::default(),
                        identifier: Identifier::new("b"),
                        type_: Type::Reference(Reference {
                            mutability: Mutability::Constant,
                            type_: Box::new(Type::Composite(Identifier::new("String").into(), Default::default()))
                        })
                    },
                    Parameter {
                        attributes: Default::default(),
                        identifier: Identifier::new("c"),
                        type_: Type::Reference(Reference {
                            mutability: Mutability::Mutable,
                            type_: Box::new(Type::Composite(Identifier::new("String").into(), Default::default()))
                        })
                    },
                ],
                output: Some(Type::Reference(Reference {
                    mutability: Mutability::Constant,
                    type_: Box::new(Type::Composite(Identifier::new("String").into(), Default::default()))
                }))
            }
        );
    }

    #[test]
    fn function_pub() {
        assert_eq!(
            Function::from(SynItemFn(parse::<syn::ItemFn>(quote! {pub fn test() {}}))),
            Function {
                attributes: Attributes { attributes: vec![] },
                visibility: Visibility::Public,
                synchrony: Synchrony::Synchronous,
                path: Identifier::new("test").into(),
                inputs: vec![],
                output: None
            }
        );
    }
}

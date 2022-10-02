use crate::prelude::*;

use crate::{Attributes, Identifier, Mutability, Parameter, Type, Visibility};

pub mod parameter;

/// Async structure.
#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct Async;

/// Method structure.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Method {
    pub mutability: Mutability,
    /// Method owner.
    pub owner: Type
}

/// Function structure.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Function {
    /// Attributes field.
    pub attributes: Attributes,
    /// Visibility field.
    pub visibility: Visibility,
    // FIXME: Rework it as owner: Option<Owner>?
    /// Method field.
    pub method: Option<Method>,
    /// Asyncness field.
    pub asyncness: Option<Async>,
    /// Identifier field.
    pub identifier: Identifier,
    /// Inputs field.
    pub inputs: Vec<Parameter>,
    /// Output field.
    pub output: Option<Type>,
}

// use syn::parse_quote::parse;
// FIXME: Remove this? We can't know the owner of the method without the Implementation block.
// #[allow(unused_qualifications)]
// impl From<TokenStream> for Function {
//     fn from(tokenstream: TokenStream) -> Self {
//         parse::<syn::ImplItemMethod>(tokenstream).into()
//     }
// }

impl From<syn::ItemFn> for Function {
    fn from(item_fn: syn::ItemFn) -> Self {
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
            method: None,
            visibility: Visibility::from(item_fn.vis),
            asyncness: match asyncness {
                Some(_x) => Some(Async),
                None => None,
            },
            identifier: ident.into(),
            inputs,
            output,
        }
    }
}

impl From<(Type, syn::ImplItemMethod)> for Function {
    fn from((owner, item_fn): (Type, syn::ImplItemMethod)) -> Self {
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
            .map(|x| x.try_into().expect("Failed to convert Parameter"))
            .collect();
        let output: Option<Type> = match output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_x, y) => {
                Some(Type::try_from(*y).expect("Failed to convert from ReturnType::Type"))
            }
        };
        let method = Some(Method {
            owner,
            mutability: Mutability::Constant // FIXME: Correctly set mutability
        });
        Self {
            attributes: Attributes {
                attributes: item_fn
                    .attrs
                    .into_iter()
                    .map(|x| x.parse_meta().expect("Failed to parse Meta").into())
                    .collect(),
            },
            method,
            visibility: Visibility::from(item_fn.vis),
            asyncness: match asyncness {
                Some(_x) => Some(Async),
                None => None,
            },
            identifier: ident.into(),
            inputs,
            output,
        }
    }
}

#[cfg(test)]
mod test {
    use quote::quote;
    use syn::parse_quote::parse;

    use crate::{Attribute, Attributes, Identifier, Literal, Mutability, Parameter, Reference, ReferenceKind, Visibility};

    use super::{Async, Function, Type};

    #[test]
    fn function() {
        assert_eq!(
            Function::from(parse::<syn::ItemFn>(quote! {fn test() {}})),
            Function {
                attributes: Attributes { attributes: vec![] },
                method: None,
                visibility: Visibility::Inherited,
                asyncness: None,
                identifier: Identifier::new("test"),
                inputs: vec![],
                output: None
            }
        );
    }

    #[test]
    fn function_impl() {
        assert_eq!(
            Function::from(parse::<syn::ItemFn>(quote! {fn test() {}})),
            Function {
                attributes: Attributes { attributes: vec![] },
                // FIXME: It doesn't make any sense here. How could we know the method owner with this test?
                method: None,
                visibility: Visibility::Inherited,
                asyncness: None,
                identifier: Identifier::new("test"),
                inputs: vec![],
                output: None
            }
        );
    }

    #[test]
    fn function_input() {
        assert_eq!(
            Function::from(parse::<syn::ItemFn>(quote! {fn test(a: String, b: String) {}})),
            Function {
                attributes: Attributes { attributes: vec![] },
                method: None,
                visibility: Visibility::Inherited,
                asyncness: None,
                identifier: Identifier::new("test"),
                inputs: vec![
                    Parameter {
                        attributes: Default::default(),
                        identifier: Identifier::new("a"),
                        type_: Type::Compound(Identifier::new("String").into(), Default::default())
                    },
                    Parameter {
                        attributes: Default::default(),
                        identifier: Identifier::new("b"),
                        type_: Type::Compound(Identifier::new("String").into(), Default::default())
                    },
                ],
                output: None
            }
        );
    }

    #[test]
    fn function_output() {
        assert_eq!(
            Function::from(parse::<syn::ItemFn>(quote! {fn test() -> String {}})),
            Function {
                attributes: Attributes { attributes: vec![] },
                method: None,
                visibility: Visibility::Inherited,
                asyncness: None,
                identifier: Identifier::new("test"),
                inputs: vec![],
                output: Some(Type::Compound(Identifier::new("String").into(), Default::default()))
            }
        );
    }

    #[test]
    fn function_input_output() {
        assert_eq!(
            Function::from(parse::<syn::ItemFn>(
                quote! {fn test(a: String, b: &String, c: &mut String) -> &String {}}
            )),
            Function {
                attributes: Attributes { attributes: vec![] },
                method: None,
                visibility: Visibility::Inherited,
                asyncness: None,
                identifier: Identifier::new("test"),
                inputs: vec![
                    Parameter {
                        attributes: Default::default(),
                        identifier: Identifier::new("a"),
                        type_: Type::Compound(Identifier::new("String").into(), Default::default())
                    },
                    Parameter {
                        attributes: Default::default(),
                        identifier: Identifier::new("b"),
                        type_: Type::Reference(Reference {
                            kind: ReferenceKind::Borrow,
                            mutability: Mutability::Constant,
                            type_: Box::new(Type::Compound(Identifier::new("String").into(), Default::default()))
                        })
                    },
                    Parameter {
                        attributes: Default::default(),
                        identifier: Identifier::new("c"),
                        type_: Type::Reference(Reference {
                            kind: ReferenceKind::Borrow,
                            mutability: Mutability::Mutable,
                            type_: Box::new(Type::Compound(Identifier::new("String").into(), Default::default()))
                        })
                    },
                ],
                output: Some(Type::Reference(Reference {
                    kind: ReferenceKind::Borrow,
                    mutability: Mutability::Constant,
                    type_: Box::new(Type::Compound(Identifier::new("String").into(), Default::default()))
                }))
            }
        );
    }

    #[test]
    fn function_attribute() {
        assert_eq!(
            Function::from(parse::<syn::ItemFn>(quote! {
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
                visibility: Visibility::Inherited,
                method: None,
                asyncness: None,
                identifier: Identifier::new("test"),
                inputs: vec![],
                output: None
            }
        );
    }

    #[test]
    fn function_async() {
        assert_eq!(
            Function::from(parse::<syn::ItemFn>(quote! {async fn test() {}})),
            Function {
                attributes: Attributes { attributes: vec![] },
                method: None,
                visibility: Visibility::Inherited,
                asyncness: Some(Async),
                identifier: Identifier::new("test"),
                inputs: vec![],
                output: None
            }
        );
    }

    #[test]
    fn function_complete() {
        assert_eq!(
            Function::from(parse::<syn::ItemFn>(quote! {
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
                visibility: Visibility::Inherited,
                method: None,
                asyncness: Some(Async),
                identifier: Identifier::new("test"),
                inputs: vec![
                    Parameter {
                        attributes: Default::default(),
                        identifier: Identifier::new("a"),
                        type_: Type::Compound(Identifier::new("String").into(), Default::default())
                    },
                    Parameter {
                        attributes: Default::default(),
                        identifier: Identifier::new("b"),
                        type_: Type::Reference(Reference {
                            kind: ReferenceKind::Borrow,
                            mutability: Mutability::Constant,
                            type_: Box::new(Type::Compound(Identifier::new("String").into(), Default::default()))
                        })
                    },
                    Parameter {
                        attributes: Default::default(),
                        identifier: Identifier::new("c"),
                        type_: Type::Reference(Reference {
                            kind: ReferenceKind::Borrow,
                            mutability: Mutability::Mutable,
                            type_: Box::new(Type::Compound(Identifier::new("String").into(), Default::default()))
                        })
                    },
                ],
                output: Some(Type::Reference(Reference {
                    kind: ReferenceKind::Borrow,
                    mutability: Mutability::Constant,
                    type_: Box::new(Type::Compound(Identifier::new("String").into(), Default::default()))
                }))
            }
        );
    }

    #[test]
    fn function_pub() {
        assert_eq!(
            Function::from(parse::<syn::ItemFn>(quote! {pub fn test() {}})),
            Function {
                attributes: Attributes { attributes: vec![] },
                // FIXME: ImplItemMethod are for methods and method None.
                method: None,
                visibility: Visibility::Public,
                asyncness: None,
                identifier: Identifier::new("test"),
                inputs: vec![],
                output: None
            }
        );
    }
}

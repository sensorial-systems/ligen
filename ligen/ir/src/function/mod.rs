use crate::prelude::*;
use syn::{ImplItemMethod, ItemFn};

use crate::{Attributes, Identifier, Mutability, Parameter, Type, Visibility};
use syn::parse_quote::parse;

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

#[allow(unused_qualifications)]
impl From<TokenStream> for Function {
    fn from(tokenstream: TokenStream) -> Self {
        parse::<syn::ImplItemMethod>(tokenstream).into()
    }
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
    };
}

impl_function!(ItemFn);
impl_function!(ImplItemMethod);

#[cfg(test)]
mod test {
    use quote::quote;
    use syn::parse_quote::parse;

    use crate::{Attribute, Attributes, Identifier, Literal, Mutability, Parameter, Reference, ReferenceKind, Visibility};

    use super::{Async, Function, ImplItemMethod, ItemFn, Type};

    #[test]
    fn function() {
        assert_eq!(
            Function::from(parse::<ItemFn>(quote! {fn test() {}})),
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
            Function::from(parse::<ImplItemMethod>(quote! {fn test() {}})),
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
            Function::from(parse::<ItemFn>(quote! {fn test(a: String, b: String) {}})),
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
            Function::from(parse::<ItemFn>(quote! {fn test() -> String {}})),
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
            Function::from(parse::<ItemFn>(
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
            Function::from(parse::<ItemFn>(quote! {async fn test() {}})),
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
            Function::from(parse::<ImplItemMethod>(quote! {pub fn test() {}})),
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

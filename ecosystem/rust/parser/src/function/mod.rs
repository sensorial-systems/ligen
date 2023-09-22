use crate::prelude::*;

use ligen_ir::{Attributes, Function, Parameter, Type};
use ligen_parsing::Parser;
use crate::function::parameter::ParameterParser;
use crate::identifier::IdentifierParser;
use crate::macro_attributes::attributes::AttributeParser;
use crate::types::TypeParser;

mod parameter;
mod method;
mod synchrony;

pub use parameter::*;
pub use method::*;
pub use synchrony::*;
use crate::visibility::VisibilityParser;

pub struct FunctionParser;

impl Parser<syn::ItemFn> for FunctionParser {
    type Output = Function;
    fn parse(&self, item_fn: syn::ItemFn) -> Result<Self::Output> {
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
            .map(|x| ParameterParser.parse(x).expect("Failed to convert Parameter"))
            .collect();
        let output: Option<Type> = match output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_x, y) => {
                Some(TypeParser.parse(*y)?)
            }
        };
        Ok(Self::Output {
            attributes: Attributes {
                attributes: item_fn
                    .attrs
                    .into_iter()
                    .map(|attribute| AttributeParser.parse(attribute).expect("Failed to parse Meta"))
                    .collect(),
            },
            visibility: VisibilityParser.parse(item_fn.vis)?,
            synchrony: SynchronyParser.parse(asyncness)?,
            path: IdentifierParser.parse(ident)?.into(),
            inputs,
            output,
        })
    }
}

// FIXME: Can we make this a subset of method? What about using the MethodParser and then just catch the things we care about.
//  This is repeating Parser<syn::ImplItemMethod> for MethodParser.
impl Parser<syn::ImplItemMethod> for FunctionParser {
    type Output = Function;
    fn parse(&self, method: syn::ImplItemMethod) -> Result<Self::Output> {
        let syn::Signature {
            asyncness,
            ident,
            inputs,
            output,
            ..
        } = method.sig;
        let inputs: Vec<Parameter> = inputs
            .clone()
            .into_iter()
            .map(|x| ParameterParser.parse(x).expect("Failed to convert Parameter"))
            .collect();
        let output: Option<Type> = match output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_x, y) => {
                Some(TypeParser.parse(*y).expect("Failed to convert from ReturnType::Type"))
            }
        };
        Ok(Self::Output {
            attributes: Attributes {
                attributes: method
                    .attrs
                    .into_iter()
                    .map(|attribute| AttributeParser.parse(attribute).expect("Failed to parse Meta"))
                    .collect(),
            },
            visibility: VisibilityParser.parse(method.vis)?,
            synchrony: SynchronyParser.parse(asyncness)?,
            path: IdentifierParser.parse(ident)?.into(),
            inputs,
            output,
        })
    }
}

impl Parser<proc_macro::TokenStream> for FunctionParser {
    type Output = Function;
    fn parse(&self, token_stream: proc_macro::TokenStream) -> Result<Self::Output> {
        self.parse(proc_macro2::TokenStream::from(token_stream))
    }
}

impl Parser<proc_macro2::TokenStream> for FunctionParser {
    type Output = Function;
    fn parse(&self, token_stream: proc_macro2::TokenStream) -> Result<Self::Output> {
        syn::parse2::<syn::ItemFn>(token_stream)
            .map_err(|e| Error::Message(format!("Failed to parse function: {:?}", e)))
            .and_then(|function| self.parse(function))
    }
}

#[cfg(test)]
mod test {
    use ligen_ir::Synchrony;
    use ligen_ir::{Attribute, Attributes, Identifier, Literal, Mutability, Parameter, Reference, Visibility};
    use ligen_parsing::Parser;
    use crate::function::FunctionParser;
    use crate::prelude::*;

    use super::{Function, Type};

    #[test]
    fn function() -> Result<()> {
        assert_eq!(
            FunctionParser.parse(quote! {fn test() {}})?,
            Function {
                attributes: Attributes { attributes: vec![] },
                visibility: Visibility::Private,
                synchrony: Synchrony::Synchronous,
                path: Identifier::new("test").into(),
                inputs: vec![],
                output: None
            }
        );
        Ok(())
    }

    #[test]
    fn function_impl() -> Result<()> {
        assert_eq!(
            FunctionParser.parse(quote! {fn test() {}})?,
            Function {
                attributes: Attributes { attributes: vec![] },
                visibility: Visibility::Private,
                synchrony: Synchrony::Synchronous,
                path: Identifier::new("test").into(),
                inputs: vec![],
                output: None
            }
        );
        Ok(())
    }

    #[test]
    fn function_input() -> Result<()> {
        assert_eq!(
            FunctionParser.parse(quote! {fn test(a: String, b: String) {}})?,
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
        Ok(())
    }

    #[test]
    fn function_output() -> Result<()> {
        assert_eq!(
            FunctionParser.parse(quote! {fn test() -> String {}})?,
            Function {
                attributes: Attributes { attributes: vec![] },
                visibility: Visibility::Private,
                synchrony: Synchrony::Synchronous,
                path: Identifier::new("test").into(),
                inputs: vec![],
                output: Some(Type::Composite(Identifier::new("String").into(), Default::default()))
            }
        );
        Ok(())
    }

    #[test]
    fn function_input_output() -> Result<()> {
        assert_eq!(
            FunctionParser.parse(quote! {fn test(a: String, b: &String, c: &mut String) -> &String {}})?,
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
        Ok(())
    }

    #[test]
    fn function_attribute() -> Result<()> {
        assert_eq!(
            FunctionParser.parse(quote! {
                #[test(a = "b")]
                fn test() {}
            })?,
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
        Ok(())
    }

    #[test]
    fn function_async() -> Result<()> {
        assert_eq!(
            FunctionParser.parse(quote! {async fn test() {}})?,
            Function {
                attributes: Attributes { attributes: vec![] },
                visibility: Visibility::Private,
                synchrony: Synchrony::Asynchronous,
                path: Identifier::new("test").into(),
                inputs: vec![],
                output: None
            }
        );
        Ok(())
    }

    #[test]
    fn function_complete() -> Result<()> {
        assert_eq!(
            FunctionParser.parse(quote! {
            #[test(a = "b")]
                async fn test(a: String, b: &String, c: &mut String) -> &String {}
            })?,
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
        Ok(())
    }

    #[test]
    fn function_pub() -> Result<()> {
        assert_eq!(
            FunctionParser.parse(quote! {pub fn test() {}})?,
            Function {
                attributes: Attributes { attributes: vec![] },
                visibility: Visibility::Public,
                synchrony: Synchrony::Synchronous,
                path: Identifier::new("test").into(),
                inputs: vec![],
                output: None
            }
        );
        Ok(())
    }
}

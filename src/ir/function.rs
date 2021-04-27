use crate::ir::{Argument, Attributes, Identifier, Type};
use syn::ItemFn;

#[derive(Debug, PartialEq)]
/// Function Struct
pub struct Function {
    identifier: Identifier,
    attributes: Option<Attributes>,
    input: Vec<Argument>,
    output: Option<Type>,
}

impl From<ItemFn> for Function {
    fn from(item_fn: ItemFn) -> Self {
        println!("{:#?}", item_fn);
        if let syn::Signature {
            ident,
            inputs,
            output,
            ..
        } = item_fn.sig
        {
            let input: Vec<Argument> = inputs
                .clone()
                .into_iter()
                .map(|x| Argument::from(x))
                .collect();
            Self {
                identifier: Identifier::from(ident),
                attributes: None,
                input,
                output: None,
            }
        } else {
            panic!("Function signature missing or incomplete");
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Function, ItemFn, Type};
    use crate::ir::{Argument, Identifier};
    use quote::quote;
    use syn::parse_quote::parse;

    #[test]
    fn function() {
        assert_eq!(
            Function::from(parse::<ItemFn>(quote! {fn test() {}})),
            Function {
                identifier: Identifier {
                    name: String::from("test")
                },
                attributes: None,
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
                identifier: Identifier {
                    name: String::from("test")
                },
                attributes: None,
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
                identifier: Identifier {
                    name: String::from("test")
                },
                attributes: None,
                input: vec![],
                output: Some(Type::Compound(Identifier {
                    name: String::from("String")
                }))
            }
        );
    }
}

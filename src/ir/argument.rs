use crate::ir::{Identifier, Type};
use syn::{Expr, ExprType};

#[derive(Debug, PartialEq)]
/// Argument Struct
pub struct Argument {
    identifier: Identifier,
    type_: Type,
}

impl From<ExprType> for Argument {
    fn from(expr_type: ExprType) -> Self {
        // println!("{:#?}", expr_type);

        let identifier = match *expr_type.expr {
            Expr::Path(syn::ExprPath { path, .. }) => path.segments[0].ident.clone(),
            _ => panic!("Identifier not found"),
        };

        Self {
            identifier: Identifier::from(identifier),
            type_: Type::from(*expr_type.ty),
        }
    }
}

#[cfg(test)]
mod test {

    use super::Argument;
    use crate::ir::{Atomic, Borrow, Identifier, Integer, Pointer, Reference, Type};
    use quote::quote;
    use syn::parse_quote::parse;

    #[test]
    fn argument_atomic() {
        assert_eq!(
            Argument::from(parse::<syn::ExprType>(quote! {integer: i32})),
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
            Argument::from(parse::<syn::ExprType>(quote! {name: String})),
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
            Argument::from(parse::<syn::ExprType>(quote! {name: &String})),
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
            Argument::from(parse::<syn::ExprType>(quote! {name: &mut String})),
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
            Argument::from(parse::<syn::ExprType>(quote! {name: *const String})),
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
            Argument::from(parse::<syn::ExprType>(quote! {name: *mut String})),
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
}

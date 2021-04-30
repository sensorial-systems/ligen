use crate::ir::{Identifier, Literal, Type};
use std::convert::TryFrom;
use syn::{ImplItemConst, ItemConst};

#[derive(Debug, PartialEq)]
/// Constant Struct
pub struct Constant {
    /// identifier field
    pub identifier: Identifier,
    /// type_ field
    pub type_: Type,
    /// literal field
    pub literal: Literal,
}

impl From<ImplItemConst> for Constant {
    fn from(item_const: ImplItemConst) -> Self {
        if let syn::Expr::Lit(syn::ExprLit { lit, .. }) = item_const.expr {
            Self {
                identifier: Identifier::from(item_const.ident.clone()),
                type_: Type::try_from(item_const.ty).expect("Failed to convert from Type"),
                literal: Literal::from(lit),
            }
        } else {
            panic!("Undefined Constant inside Impl block");
        }
    }
}

impl From<ItemConst> for Constant {
    fn from(item_const: ItemConst) -> Self {
        if let syn::Expr::Lit(syn::ExprLit { lit, .. }) = *item_const.expr {
            Self {
                identifier: Identifier::from(item_const.ident.clone()),
                type_: Type::try_from(*item_const.ty).expect("Failed to convert from Type"),
                literal: Literal::from(lit),
            }
        } else {
            panic!("Undefined Constant inside Impl block");
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Constant, Identifier, ImplItemConst, ItemConst, Type};
    use crate::ir::{Borrow, Literal, Reference};
    use quote::quote;
    use syn::parse_quote::parse;

    #[test]
    fn impl_const_impl() {
        assert_eq!(
            Constant::from(parse::<ImplItemConst>(quote! {const a: &str = "teste";})),
            Constant {
                identifier: Identifier {
                    name: String::from("a")
                },
                type_: Type::Reference(Reference::Borrow(Borrow::Constant(Box::new(
                    Type::Compound(Identifier {
                        name: String::from("str")
                    })
                )))),
                literal: Literal::String(String::from("teste"))
            }
        );
    }

    #[test]
    fn impl_const() {
        assert_eq!(
            Constant::from(parse::<ItemConst>(quote! {const a: &str = "teste";})),
            Constant {
                identifier: Identifier {
                    name: String::from("a")
                },
                type_: Type::Reference(Reference::Borrow(Borrow::Constant(Box::new(
                    Type::Compound(Identifier {
                        name: String::from("str")
                    })
                )))),
                literal: Literal::String(String::from("teste"))
            }
        );
    }
}

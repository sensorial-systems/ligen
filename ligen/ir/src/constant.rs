use crate::{Identifier, Literal, Type};
use crate::prelude::*;
use syn::{ImplItemConst, ItemConst};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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
            panic!("Undefined Constant");
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Constant, Identifier, ImplItemConst, ItemConst, Type};
    use crate::{Literal, Mutability, Reference, ReferenceKind};
    use quote::quote;
    use syn::parse_quote::parse;

    #[test]
    fn impl_const_impl() {
        assert_eq!(
            Constant::from(parse::<ImplItemConst>(quote! {const a: &str = "test";})),
            Constant {
                identifier: Identifier::new("a"),
                type_: Type::Reference(
                    Reference {
                        kind: ReferenceKind::Borrow,
                        mutability: Mutability::Constant,
                        type_: Box::new(Type::Compound(Identifier::new("str").into(), Default::default()))
                    }
                ),
                literal: Literal::String(String::from("test"))
            }
        );
    }

    #[test]
    fn impl_const() {
        assert_eq!(
            Constant::from(parse::<ItemConst>(quote! {const a: &str = "test";})),
            Constant {
                identifier: Identifier::new("a"),
                type_: Type::Reference(
                    Reference {
                        kind: ReferenceKind::Borrow,
                        mutability: Mutability::Constant,
                        type_: Box::new(Type::Compound(Identifier::new("str").into(), Default::default()))
                    }
                ),
                literal: Literal::String(String::from("test"))
            }
        );
    }
}

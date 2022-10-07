use crate::{Constant, Identifier, Literal, Type};
use crate::prelude::*;

// TODO: Should be TryFrom?
impl From<SynImplItemConst> for Constant {
    fn from(SynImplItemConst(item_const): SynImplItemConst) -> Self {
        if let syn::Expr::Lit(syn::ExprLit { lit, .. }) = item_const.expr {
            Self {
                identifier: Identifier::from(SynIdent(item_const.ident.clone())),
                type_: Type::try_from(SynType(item_const.ty)).expect("Failed to convert from Type"),
                literal: Literal::from(SynLit(lit)),
            }
        } else {
            panic!("Undefined Constant inside Impl block");
        }
    }
}

impl From<SynItemConst> for Constant {
    fn from(SynItemConst(item_const): SynItemConst) -> Self {
        if let syn::Expr::Lit(syn::ExprLit { lit, .. }) = *item_const.expr {
            Self {
                identifier: Identifier::from(SynIdent(item_const.ident.clone())),
                type_: Type::try_from(SynType(*item_const.ty)).expect("Failed to convert from Type"),
                literal: Literal::from(SynLit(lit)),
            }
        } else {
            panic!("Undefined Constant");
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Constant, Identifier, Type};
    use crate::{Literal, Mutability, Reference, ReferenceKind};
    use quote::quote;
    use syn::parse_quote::parse;
    use crate::prelude::{SynImplItemConst, SynItemConst};

    #[test]
    fn impl_const_impl() {
        assert_eq!(
            Constant::from(SynImplItemConst(parse::<syn::ImplItemConst>(quote! {const a: &str = "test";}))),
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
            Constant::from(SynItemConst(parse::<syn::ItemConst>(quote! {const a: &str = "test";}))),
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

use crate::{Constant, Identifier, Literal, Type};
use crate::prelude::*;

impl TryFrom<SynImplItemConst> for Constant {
    type Error = Error;
    fn try_from(SynImplItemConst(item_const): SynImplItemConst) -> Result<Self> {
        if let syn::Expr::Lit(syn::ExprLit { lit, .. }) = item_const.expr {
            Ok(Self {
                path: Identifier::from(SynIdent(item_const.ident.clone())).into(),
                type_: Type::try_from(SynType(item_const.ty))?,
                literal: Literal::from(SynLit(lit)),
            })
        } else {
            Err("Undefined Constant inside Impl block".into())
        }
    }
}

impl TryFrom<SynItemConst> for Constant {
    type Error = Error;
    fn try_from(SynItemConst(item_const): SynItemConst) -> Result<Self> {
        if let syn::Expr::Lit(syn::ExprLit { lit, .. }) = *item_const.expr {
            Ok(Self {
                path: Identifier::from(SynIdent(item_const.ident.clone())).into(),
                type_: Type::try_from(SynType(*item_const.ty))?,
                literal: Literal::from(SynLit(lit)),
            })
        } else {
            Err("Undefined Constant".into())
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Constant, Identifier, Type};
    use crate::{Literal, Mutability, Reference};
    use quote::quote;
    use syn::parse_quote::parse;
    use crate::prelude::*;

    #[test]
    fn impl_const_impl() -> Result<()> {
        assert_eq!(
            Constant::try_from(SynImplItemConst(parse::<syn::ImplItemConst>(quote! {const a: &str = "test";})))?,
            Constant {
                path: "a".into(),
                type_: Type::Reference(
                    Reference {
                        mutability: Mutability::Constant,
                        type_: Box::new(Type::Composite(Identifier::new("str").into(), Default::default()))
                    }
                ),
                literal: Literal::String(String::from("test"))
            }
        );
        Ok(())
    }

    #[test]
    fn impl_const() -> Result<()> {
        assert_eq!(
            Constant::try_from(SynItemConst(parse::<syn::ItemConst>(quote! {const a: &str = "test";})))?,
            Constant {
                path: "a".into(),
                type_: Type::Reference(
                    Reference {
                        mutability: Mutability::Constant,
                        type_: Box::new(Type::Composite(Identifier::new("str").into(), Default::default()))
                    }
                ),
                literal: Literal::String(String::from("test"))
            }
        );
        Ok(())
    }
}

use ligen_ir::Constant;
use ligen_parsing::Parser;
use crate::identifier::IdentifierParser;
use crate::literal::LiteralParser;
use crate::prelude::*;
use crate::types::TypeParser;

pub struct ConstantParser;

impl Parser<syn::ImplItemConst> for ConstantParser {
    type Output = Constant;
    fn parse(&self, item_const: syn::ImplItemConst) -> Result<Self::Output> {
        if let syn::Expr::Lit(syn::ExprLit { lit, .. }) = item_const.expr {
            Ok(Self::Output {
                path: IdentifierParser.parse(item_const.ident.clone())?.into(),
                type_: TypeParser.parse(item_const.ty)?,
                literal: LiteralParser.parse(lit)?,
            })
        } else {
            Err("Undefined Constant inside Impl block".into())
        }
    }
}

impl Parser<syn::ItemConst> for ConstantParser {
    type Output = Constant;
    fn parse(&self, item_const: syn::ItemConst) -> Result<Self::Output> {
        if let syn::Expr::Lit(syn::ExprLit { lit, .. }) = *item_const.expr {
            Ok(Self::Output {
                path: IdentifierParser.parse(item_const.ident.clone())?.into(),
                type_: TypeParser.parse(*item_const.ty)?,
                literal: LiteralParser.parse(lit)?,
            })
        } else {
            Err("Undefined Constant".into())
        }
    }
}

impl Parser<proc_macro::TokenStream> for ConstantParser {
    type Output = Constant;
    fn parse(&self, input: proc_macro::TokenStream) -> Result<Self::Output> {
        let token_stream = proc_macro2::TokenStream::from(input);
        self.parse(token_stream)
    }
}

impl Parser<proc_macro2::TokenStream> for ConstantParser {
    type Output = Constant;
    fn parse(&self, input: proc_macro2::TokenStream) -> Result<Self::Output> {
        let constant = syn::parse2::<syn::ItemConst>(input).expect("Failed to parse constant.");
        self.parse(constant)
    }
}

#[cfg(test)]
mod test {
    use ligen_ir::{Literal, Mutability, Reference, Constant, Identifier, Type};
    use quote::quote;
    use crate::constant::ConstantParser;
    use crate::prelude::*;

    #[test]
    fn impl_const_impl() -> Result<()> {
        assert_eq!(
            ConstantParser.parse(quote! {const a: &str = "test";})?,
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
            ConstantParser.parse(quote! {const a: &str = "test";})?,
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

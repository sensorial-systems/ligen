use ligen::ir::Constant;
use ligen::parsing::parser::Parser;
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
                identifier: IdentifierParser.parse(item_const.ident.clone())?,
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
                identifier: IdentifierParser.parse(item_const.ident.clone())?,
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
        self.parse(proc_macro2::TokenStream::from(input))
    }
}

impl Parser<proc_macro2::TokenStream> for ConstantParser {
    type Output = Constant;
    fn parse(&self, input: proc_macro2::TokenStream) -> Result<Self::Output> {
        syn::parse2::<syn::ItemConst>(input)
            .map_err(|e| Error::Message(format!("Failed to parse constant: {:?}", e)))
            .and_then(|constant| self.parse(constant))
    }
}

#[cfg(test)]
mod test {
    use quote::quote;
    use crate::constant::ConstantParser;
    use crate::prelude::*;
    
    use ligen::parsing::assert::assert_eq;
    use ligen::ir::constant::mock;
    
    #[test]
    fn constant() -> Result<()> {
        assert_eq(ConstantParser, mock::constant(), quote! {
            const CONSTANT: bool = false;
        })
    }
}

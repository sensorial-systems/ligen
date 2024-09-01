use ligen::ir::{Object, Mutability};
use ligen::parser::{Parser, ParserConfig};
use crate::identifier::IdentifierParser;
use crate::literal::LiteralParser;
use crate::prelude::*;
use crate::types::TypeParser;

#[derive(Default)]
pub struct ObjectParser;

impl ObjectParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Parser<syn::ImplItemConst> for ObjectParser {
    type Output = Object;
    fn parse(&self, item_const: syn::ImplItemConst, config: &ParserConfig) -> Result<Self::Output> {
        if let syn::Expr::Lit(syn::ExprLit { lit, .. }) = item_const.expr {
            let mutability = Mutability::Constant;
            let identifier = IdentifierParser::new().parse(item_const.ident.clone(), config)?;
            let type_ = TypeParser::new().parse(item_const.ty, config)?;
            let literal = LiteralParser.parse(lit, config)?;
            Ok(Self::Output { mutability, identifier, type_, literal })
        } else {
            Err("Undefined Constant inside Impl block".into())
        }
    }
}

impl Parser<syn::ItemConst> for ObjectParser {
    type Output = Object;
    fn parse(&self, item_const: syn::ItemConst, config: &ParserConfig) -> Result<Self::Output> {
        if let syn::Expr::Lit(syn::ExprLit { lit, .. }) = *item_const.expr {
            let mutability = Mutability::Constant;
            let identifier = IdentifierParser::new().parse(item_const.ident.clone(), config)?;
            let type_ = TypeParser::new().parse(*item_const.ty, config)?;
            let literal = LiteralParser.parse(lit, config)?;
            Ok(Self::Output { mutability, identifier, type_, literal })
        } else {
            Err("Undefined Constant".into())
        }
    }
}

impl Parser<proc_macro::TokenStream> for ObjectParser {
    type Output = Object;
    fn parse(&self, input: proc_macro::TokenStream, config: &ParserConfig) -> Result<Self::Output> {
        self.parse(proc_macro2::TokenStream::from(input), config)
    }
}

impl Parser<proc_macro2::TokenStream> for ObjectParser {
    type Output = Object;
    fn parse(&self, input: proc_macro2::TokenStream, config: &ParserConfig) -> Result<Self::Output> {
        syn::parse2::<syn::ItemConst>(input)
            .map_err(|e| Error::Message(format!("Failed to parse constant: {:?}", e)))
            .and_then(|constant| self.parse(constant, config))
    }
}

#[cfg(test)]
mod test {
    use quote::quote;
    use crate::object::ObjectParser;
    use crate::prelude::*;
    
    use ligen::parser::assert::assert_eq;
    use ligen::ir::object::mock;
    
    #[test]
    fn constant() -> Result<()> {
        assert_eq(ObjectParser, mock::constant(), quote! {
            const CONSTANT: bool = false;
        })
    }

    // TODO: Add test for static.
}

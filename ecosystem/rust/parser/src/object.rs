use ligen::idl::{Object, Mutability};
use ligen::transformer::prelude::*;
use crate::identifier::RustIdentifierParser;
use crate::literal::RustLiteralParser;
use crate::types::RustTypeParser;

#[derive(Default)]
pub struct RustObjectParser {
    identifier_parser: RustIdentifierParser,
    type_parser: RustTypeParser,
    literal_parser: RustLiteralParser,
}

impl RustObjectParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Transformer<syn::ImplItemConst, Object> for RustObjectParser {
    fn transform(&self, item_const: syn::ImplItemConst, config: &Config) -> Result<Object> {
        if let syn::Expr::Lit(syn::ExprLit { lit, .. }) = item_const.expr {
            let mutability = Mutability::Constant;
            let identifier = self.identifier_parser.transform(item_const.ident.clone(), config)?;
            let type_ = self.type_parser.transform(item_const.ty, config)?;
            let literal = self.literal_parser.transform(lit, config)?;
            Ok(Object { mutability, identifier, type_, literal })
        } else {
            Err("Undefined Constant inside Impl block".into())
        }
    }
}

impl Transformer<syn::ItemConst, Object> for RustObjectParser {
    fn transform(&self, item_const: syn::ItemConst, config: &Config) -> Result<Object> {
        if let syn::Expr::Lit(syn::ExprLit { lit, .. }) = *item_const.expr {
            let mutability = Mutability::Constant;
            let identifier = self.identifier_parser.transform(item_const.ident.clone(), config)?;
            let type_ = self.type_parser.transform(*item_const.ty, config)?;
            let literal = self.literal_parser.transform(lit, config)?;
            Ok(Object { mutability, identifier, type_, literal })
        } else {
            Err("Undefined Constant".into())
        }
    }
}

impl Transformer<proc_macro::TokenStream, Object> for RustObjectParser {
    fn transform(&self, input: proc_macro::TokenStream, config: &Config) -> Result<Object> {
        self.transform(proc_macro2::TokenStream::from(input), config)
    }
}

impl Transformer<proc_macro2::TokenStream, Object> for RustObjectParser {
    fn transform(&self, input: proc_macro2::TokenStream, config: &Config) -> Result<Object> {
        syn::parse2::<syn::ItemConst>(input)
            .map_err(|e| Error::Message(format!("Failed to parse constant: {e:?}")))
            .and_then(|constant| self.transform(constant, config))
    }
}

#[cfg(test)]
mod test {
    use quote::quote;
    use crate::object::RustObjectParser;
    use crate::prelude::*;
    
    use ligen::transformer::assert::assert_eq;
    use ligen::idl::object::mock;
    
    #[test]
    fn constant() -> Result<()> {
        assert_eq(RustObjectParser::default(), mock::constant(), quote! {
            const CONSTANT: bool = false;
        })
    }

    // TODO: Add test for static.
}

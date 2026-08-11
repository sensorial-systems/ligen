use crate::identifier::RustIdentifierParser;
use crate::literal::RustLiteralParser;
use crate::types::RustTypeParser;
use crate::RustVisibilityParser;
use ligen::idl::{Mutability, Object};
use ligen::transformer::prelude::*;

#[derive(Default)]
pub struct RustObjectParser {
    identifier_parser: RustIdentifierParser,
    type_parser: RustTypeParser,
    literal_parser: RustLiteralParser,
    visibility_parser: RustVisibilityParser,
}

impl RustObjectParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Transformer<syn::ImplItemConst, Object> for RustObjectParser {
    fn transform(&self, item_const: syn::ImplItemConst, config: &Config) -> Result<Object> {
        match item_const.expr {
            syn::Expr::Array(syn::ExprArray { elems, .. }) => {
                let mutability = Mutability::Constant;
                let visibility = self.visibility_parser.transform(item_const.vis, config)?;
                let identifier = self
                    .identifier_parser
                    .transform(item_const.ident.clone(), config)?;
                let type_ = self.type_parser.transform(item_const.ty, config)?;
                let literal = self.literal_parser.transform(elems, config)?;
                Ok(Object {
                    visibility,
                    mutability,
                    identifier,
                    type_,
                    literal,
                })
            }
            syn::Expr::Lit(syn::ExprLit { lit, .. }) => {
                let mutability = Mutability::Constant;
                let visibility = self.visibility_parser.transform(item_const.vis, config)?;
                let identifier = self
                    .identifier_parser
                    .transform(item_const.ident.clone(), config)?;
                let type_ = self.type_parser.transform(item_const.ty, config)?;
                let literal = self.literal_parser.transform(lit, config)?;
                Ok(Object {
                    visibility,
                    mutability,
                    identifier,
                    type_,
                    literal,
                })
            }
            _ => Err("Undefined Constant inside Impl block".into()),
        }
    }
}

impl Transformer<syn::ItemConst, Object> for RustObjectParser {
    /// A `const` item, with its value worked out.
    ///
    /// The value is whatever the literal parser can fold the initializer down to, which is more
    /// than a literal: see its `syn::Expr` transformer. This used to accept a literal and a list of
    /// them and nothing else, so a constant written as the arithmetic it is — or as `[0; 32]` —
    /// stopped the whole IDL, and a program had to spell its constants out with a comment saying
    /// why.
    fn transform(&self, item_const: syn::ItemConst, config: &Config) -> Result<Object> {
        let mutability = Mutability::Constant;
        let visibility = self.visibility_parser.transform(item_const.vis, config)?;
        let identifier = self
            .identifier_parser
            .transform(item_const.ident.clone(), config)?;
        let type_ = self.type_parser.transform(*item_const.ty, config)?;
        let literal = self
            .literal_parser
            .transform(*item_const.expr, config)
            .map_err(|error| Error::Message(format!("Constant {}: {error}", item_const.ident)))?;
        Ok(Object {
            visibility,
            mutability,
            identifier,
            type_,
            literal,
        })
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
    use crate::object::RustObjectParser;
    use crate::prelude::*;
    use quote::quote;

    use ligen::idl::object::mock;
    use ligen::transformer::assert::assert_eq;

    #[test]
    fn constant() -> Result<()> {
        assert_eq(
            RustObjectParser::default(),
            mock::constant(),
            quote! {
                const CONSTANT: bool = false;
            },
        )
    }

    // TODO: Add test for static.
}

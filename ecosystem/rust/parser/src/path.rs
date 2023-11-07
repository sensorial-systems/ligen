use ligen::ir::{Path, PathSegment};
use ligen::parser::{Parser, ParserConfig};
use crate::identifier::IdentifierParser;
use crate::prelude::*;

#[derive(Default)]
pub struct PathParser {}

impl PathParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Parser<syn::Path> for PathParser {
    type Output = Path;
    fn parse(&self, path: syn::Path, config: &ParserConfig) -> Result<Self::Output> {
        let segments = path
            .segments
            .iter()
            // FIXME: This isn't parsing generics, just the identifiers.
            .map(|segment| IdentifierParser::new().parse(segment.ident.clone(), config).expect("Failed to parse segment."))
            .map(PathSegment::from)
            .collect();
        Ok(Self::Output { segments })
    }
}

impl Parser<syn::Ident> for PathParser {
    type Output = Path;
    fn parse(&self, identifier: syn::Ident, config: &ParserConfig) -> Result<Self::Output> {
        let segments = vec![IdentifierParser::new().parse(identifier, config)?.into()];
        Ok(Self::Output { segments })
    }
}

impl Parser<proc_macro::TokenStream> for PathParser {
    type Output = Path;
    fn parse(&self, input: proc_macro::TokenStream, config: &ParserConfig) -> Result<Self::Output> {
        self.parse(proc_macro2::TokenStream::from(input), config)
    }
}

impl Parser<proc_macro2::TokenStream> for PathParser {
    type Output = Path;
    fn parse(&self, input: proc_macro2::TokenStream, config: &ParserConfig) -> Result<Self::Output> {
        syn::parse2::<syn::Path>(input)
            .map_err(|e| Error::Message(format!("Failed to parse path: {:?}", e)))
            .and_then(|path| self.parse(path, config))
    }
}

#[cfg(test)]
mod test {
    use crate::path::PathParser;
    use crate::prelude::*;

    use ligen::parser::assert::*;
    use ligen::ir::path::mock;

    #[test]
    fn identifier_as_path() -> Result<()> {
        assert_eq(PathParser::default(), mock::identifier_as_path(), quote! {
            u8
        })
    }

    #[test]
    fn path() -> Result<()> {
        assert_eq(PathParser::default(), mock::path(), quote! {
            std::convert::TryFrom
        })
    }
}
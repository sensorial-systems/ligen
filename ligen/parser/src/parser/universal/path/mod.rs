// FIXME: This is duplicated from Rust's parser.

use ligen_ir::{Path, PathSegment};
use crate::{Parser, ParserConfig};
use crate::universal::identifier::IdentifierParser;
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
            .map(|segment| IdentifierParser::new().parse(segment.ident.clone(), config).expect("Failed to parse segment.")) // FIXME: Remove this expect.
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

impl Parser<&str> for PathParser {
    type Output = Path;
    fn parse(&self, input: &str, config: &ParserConfig) -> Result<Self::Output> {
        syn::parse_str::<syn::Path>(input)
            .map_err(|e| Error::Message(format!("Failed to parse path: {:?}", e)))
            .and_then(|path| self.parse(path, config))
    }
}

#[cfg(test)]
mod test {
    use super::PathParser;
    use crate::prelude::*;

    use crate::assert::*;
    use ligen_ir::path::mock;

    #[test]
    fn identifier_as_path() -> Result<()> {
        assert_eq(PathParser::default(), mock::identifier_as_path(), "u8")
    }

    #[test]
    fn path() -> Result<()> {
        assert_eq(PathParser::default(), mock::path(), "std::convert::TryFrom")
    }
}
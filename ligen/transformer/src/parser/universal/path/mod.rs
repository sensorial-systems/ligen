// FIXME: This is duplicated from Rust's parser.

use crate::parser::*;

use ligen_ir::{Path, PathSegment};
use crate::parser::universal::identifier::IdentifierParser;

#[derive(Default)]
pub struct PathParser {}

impl PathParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Transformer<syn::Path, Path> for PathParser {
    fn transform(&self, path: syn::Path, config: &Config) -> Result<Path> {
        let segments = path
            .segments
            .iter()
            // FIXME: This isn't parsing generics, just the identifiers.
            .map(|segment| IdentifierParser::new().transform(segment.ident.clone(), config).expect("Failed to parse segment.")) // FIXME: Remove this expect.
            .map(PathSegment::from)
            .collect();
        Ok(Path { segments })
    }
}

impl Transformer<syn::Ident, Path> for PathParser {
    fn transform(&self, identifier: syn::Ident, config: &Config) -> Result<Path> {
        let segments = vec![IdentifierParser::new().transform(identifier, config)?.into()];
        Ok(Path { segments })
    }
}

impl Transformer<&str, Path> for PathParser {
    fn transform(&self, input: &str, config: &Config) -> Result<Path> {
        syn::parse_str::<syn::Path>(input)
            .map_err(|e| Error::Message(format!("Failed to parse path: {:?}", e)))
            .and_then(|path| self.transform(path, config))
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
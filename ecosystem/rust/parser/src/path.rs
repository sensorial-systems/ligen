use ligen::ir::{Path, PathSegment};
use ligen::parser::prelude::*;
use crate::identifier::IdentifierParser;

#[derive(Default)]
pub struct PathParser {
    identifier_parser: IdentifierParser,
}

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
            .map(|segment| self.identifier_parser.transform(segment.ident.clone(), config).expect("Failed to parse segment.")) // FIXME: Remove this expect.
            .map(PathSegment::from)
            .collect();
        Ok(Path { segments })
    }
}

impl Transformer<syn::Ident, Path> for PathParser {
    fn transform(&self, identifier: syn::Ident, config: &Config) -> Result<Path> {
        let segments = vec![self.identifier_parser.transform(identifier, config)?.into()];
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

impl Transformer<proc_macro::TokenStream, Path> for PathParser {
    fn transform(&self, input: proc_macro::TokenStream, config: &Config) -> Result<Path> {
        self.transform(proc_macro2::TokenStream::from(input), config)
    }
}

impl Transformer<proc_macro2::TokenStream, Path> for PathParser {
    fn transform(&self, input: proc_macro2::TokenStream, config: &Config) -> Result<Path> {
        syn::parse2::<syn::Path>(input)
            .map_err(|e| Error::Message(format!("Failed to parse path: {:?}", e)))
            .and_then(|path| self.transform(path, config))
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
        assert_eq(PathParser::default(), mock::identifier_as_path(), "u8")
    }

    #[test]
    fn path() -> Result<()> {
        assert_eq(PathParser::default(), mock::path(), "std::convert::TryFrom")
    }
}
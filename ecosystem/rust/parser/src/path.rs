use ligen::idl::{Path, PathSegment};
use ligen::transformer::prelude::*;
use crate::{RustIdentifierParser, RustGenericsParser};

#[derive(Default)]
pub struct RustPathParser {
    identifier_parser: RustIdentifierParser,
}

impl RustPathParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Transformer<syn::Path, Path> for RustPathParser {
    fn transform(&self, path: syn::Path, config: &Config) -> Result<Path> {
        let segments = path
            .segments
            .iter()
            .map(|segment| self.transform(segment.clone(), config).expect("Failed to parse segment.")) // FIXME: Remove this expect.
            .collect();
        Ok(Path { segments })
    }
}

impl Transformer<syn::PathSegment, PathSegment> for RustPathParser {
    fn transform(&self, input: syn::PathSegment, config: &Config) -> Result<PathSegment> {
        let identifier = self.identifier_parser.transform(input.ident, config)?;
        let generics = RustGenericsParser::default().transform(input.arguments, config)?;
        Ok(PathSegment::new(identifier, generics))
    }
}

impl Transformer<syn::Ident, Path> for RustPathParser {
    fn transform(&self, identifier: syn::Ident, config: &Config) -> Result<Path> {
        let segments = vec![self.identifier_parser.transform(identifier, config)?.into()];
        Ok(Path { segments })
    }
}

impl Transformer<&str, Path> for RustPathParser {
    fn transform(&self, input: &str, config: &Config) -> Result<Path> {
        syn::parse_str::<syn::Path>(input)
            .map_err(|e| Error::Message(format!("Failed to parse path: {e:?}")))
            .and_then(|path| self.transform(path, config))
    }
}

impl Transformer<proc_macro::TokenStream, Path> for RustPathParser {
    fn transform(&self, input: proc_macro::TokenStream, config: &Config) -> Result<Path> {
        self.transform(proc_macro2::TokenStream::from(input), config)
    }
}

impl Transformer<proc_macro2::TokenStream, Path> for RustPathParser {
    fn transform(&self, input: proc_macro2::TokenStream, config: &Config) -> Result<Path> {
        syn::parse2::<syn::Path>(input)
            .map_err(|e| Error::Message(format!("Failed to parse path: {e:?}")))
            .and_then(|path| self.transform(path, config))
    }
}

#[cfg(test)]
mod test {
    use crate::path::RustPathParser;
    use crate::prelude::*;

    use ligen::transformer::assert::*;
    use ligen::idl::path::mock;

    #[test]
    fn identifier_as_path() -> Result<()> {
        assert_eq(RustPathParser::default(), mock::identifier_as_path(), "u8")
    }

    #[test]
    fn path() -> Result<()> {
        assert_eq(RustPathParser::default(), mock::path(), "std::convert::TryFrom")
    }
}
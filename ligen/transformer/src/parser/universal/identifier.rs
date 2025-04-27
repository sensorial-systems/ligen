use crate::prelude::*;

use ligen_ir::Identifier;
use crate::parser::Parser;

#[derive(Default)]
pub struct IdentifierParser;

impl IdentifierParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Parser<Identifier> for IdentifierParser {
    fn parse(&self, input: impl AsRef<str>, _config: &Config) -> Result<Identifier> {
        // TODO: check if ident is valid identifier.
        let name = input.as_ref().into();
        Ok(Identifier { name })
    }
}

impl Transformer<&std::path::Path, Identifier> for IdentifierParser {
    fn transform(&self, input: &std::path::Path, config: &Config) -> Result<Identifier> {
        let identifier = input
            .file_stem()
            .ok_or(Error::Message(format!("Failed to parse file stem from path: {}", input.display())))?
            .to_str()
            .ok_or(Error::Message(format!("Failed to parse file stem to string: {}", input.display())))?;
        self.parse(identifier, config)

    }
}

impl Transformer<syn::Ident, Identifier> for IdentifierParser {
    fn transform(&self, ident: syn::Ident, _config: &Config) -> Result<Identifier> {
        let name = ident.to_string();
        Ok(Identifier { name })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert::*;
    use ligen_ir::identifier::mock;

    #[test]
    fn identifier() -> Result<()> {
        assert_eq(IdentifierParser, mock::identifier(), "identifier")
    }
}

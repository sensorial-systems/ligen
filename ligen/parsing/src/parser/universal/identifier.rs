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

impl Parser<String> for IdentifierParser {
    type Output = Identifier;
    fn parse(&self, input: String) -> Result<Self::Output> {
        self.parse(input.as_str())
    }
}

impl Parser<&str> for IdentifierParser {
    type Output = Identifier;
    fn parse(&self, input: &str) -> Result<Self::Output> {
        // TODO: check if ident is valid identifier.
        let name = input.into();
        Ok(Identifier { name })
    }
}

impl Parser<&std::path::Path> for IdentifierParser {
    type Output = Identifier;
    fn parse(&self, input: &std::path::Path) -> Result<Self::Output> {
        let identifier = input
            .file_stem()
            .ok_or(Error::Message(format!("Failed to parse file stem from path: {}", input.display())))?
            .to_str()
            .ok_or(Error::Message(format!("Failed to parse file stem to string: {}", input.display())))?;
        self.parse(identifier)

    }
}

impl Parser<syn::Ident> for IdentifierParser {
    type Output = Identifier;
    fn parse(&self, ident: syn::Ident) -> Result<Self::Output> {
        let name = ident.to_string();
        Ok(Self::Output { name })
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

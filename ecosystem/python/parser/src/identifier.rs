use ligen::ir::Identifier;
use ligen::parsing::parser::Parser;
use ligen::parsing::parser::universal::identifier::IdentifierParser as InternalParser;
use crate::prelude::*;

#[derive(Default)]
pub struct IdentifierParser {
    parser: InternalParser
}

impl IdentifierParser {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn parse<T>(&self, input: T) -> Result<Identifier>
    where InternalParser: Parser<T, Output = Identifier>
    {
        self
            .parser
            .parse(input)
            .and_then(|identifier| if !self.is_private(&identifier) {
                Ok(identifier)
            } else {
                Err(Error::Message(format!("Identifier {} is private", identifier.name)))
            })
    }

    pub fn is_private(&self, identifier: &Identifier) -> bool {
        identifier.name.starts_with('_') && !identifier.name.starts_with("__")
    }
}
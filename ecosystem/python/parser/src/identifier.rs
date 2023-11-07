use ligen::ir::{Identifier, Mutability};
use ligen::parser::{Parser, ParserConfig};
use ligen::parser::universal::identifier::IdentifierParser as InternalParser;
use crate::prelude::*;

#[derive(Default)]
pub struct IdentifierParser {
    parser: InternalParser
}

impl IdentifierParser {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn parse<T>(&self, input: T, config: &ParserConfig) -> Result<Identifier>
    where InternalParser: Parser<T, Output = Identifier>
    {
        self
            .parser
            .parse(input, config)
    }

    pub fn is_private(&self, identifier: &Identifier) -> bool {
        identifier.name.starts_with('_') && !identifier.name.starts_with("__")
    }

    pub fn get_mutability(&self, identifier: &Identifier) -> Mutability {
        if identifier.name.to_uppercase() == identifier.name {
            Mutability::Constant
        } else {
            Mutability::Mutable
        }
    }
}
use ligen::ir::{Identifier, Mutability, Visibility};
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

    pub fn get_visibility(&self, identifier: &Identifier) -> Visibility {
        if identifier.name.starts_with('_') && !identifier.name.starts_with("__") {
            Visibility::Private
        } else {
            Visibility::Public
        }
    }

    pub fn get_mutability(&self, identifier: &Identifier) -> Mutability {
        if identifier.name.to_uppercase() == identifier.name {
            Mutability::Constant
        } else {
            Mutability::Mutable
        }
    }
}
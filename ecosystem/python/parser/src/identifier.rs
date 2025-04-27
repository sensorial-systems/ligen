use ligen::ir::{Identifier, Mutability, Visibility};
use ligen::transformer::prelude::*;
use ligen::parser::universal::IdentifierParser as InternalParser;

#[derive(Default)]
pub struct IdentifierParser {
    parser: InternalParser
}

impl IdentifierParser {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn transform<T>(&self, input: T, config: &Config) -> Result<Identifier>
    where InternalParser: Transformer<T, Identifier>
    {
        self
            .parser
            .transform(input, config)
    }

    pub fn parse(&self, input: impl AsRef<str>, config: &Config) -> Result<Identifier> {
        self.parser.parse(input, config)
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
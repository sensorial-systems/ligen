//! Mutability enumeration.

use crate::prelude::*;
use ligen::{ir::Mutability, parser::ParserConfig};

#[derive(Default)]
pub struct MutabilityParser;

impl MutabilityParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Parser<Option<syn::token::Mut>> for MutabilityParser {
    type Output = Mutability;
    fn parse(&self, mutability: Option<syn::token::Mut>, _config: &ParserConfig) -> Result<Self::Output> {
        if mutability.is_some() {
            Ok(Mutability::Mutable)
        } else {
            Ok(Mutability::Constant)
        }
    }
}

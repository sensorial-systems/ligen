//! Mutability enumeration.

use crate::prelude::*;
use ligen::{ir::Mutability, parser::prelude::*};

#[derive(Default)]
pub struct MutabilityParser;

impl MutabilityParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Transformer<Option<syn::token::Mut>, Mutability> for MutabilityParser {
    fn transform(&self, mutability: Option<syn::token::Mut>, _config: &Config) -> Result<Mutability> {
        if mutability.is_some() {
            Ok(Mutability::Mutable)
        } else {
            Ok(Mutability::Constant)
        }
    }
}

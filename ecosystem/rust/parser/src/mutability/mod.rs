//! Mutability enumeration.

use crate::prelude::*;
use ligen::ir::Mutability;

#[derive(Default)]
pub struct RustMutabilityParser;

impl RustMutabilityParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Transformer<Option<syn::token::Mut>, Mutability> for RustMutabilityParser {
    fn transform(&self, mutability: Option<syn::token::Mut>, _config: &Config) -> Result<Mutability> {
        if mutability.is_some() {
            Ok(Mutability::Mutable)
        } else {
            Ok(Mutability::Constant)
        }
    }
}

//! Visibility enumeration.

use crate::prelude::*;
use ligen::{ir::Visibility, parser::prelude::*};

#[derive(Default)]
pub struct VisibilityParser;

impl VisibilityParser {
    pub fn new() -> Self {
        Self
    }
}

impl Transformer<syn::Visibility, Visibility> for VisibilityParser {
    fn transform(&self, visibility: syn::Visibility, _config: &Config) -> Result<Visibility> {
        Ok(match visibility {
            syn::Visibility::Public(_) => Visibility::Public,
            _ => Visibility::Private
        })
    }
}

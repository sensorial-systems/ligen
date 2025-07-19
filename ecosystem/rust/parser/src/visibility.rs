//! Visibility enumeration.

use crate::prelude::*;
use ligen::idl::Visibility;

#[derive(Default)]
pub struct RustVisibilityParser;

impl RustVisibilityParser {
    pub fn new() -> Self {
        Self
    }
}

impl Transformer<syn::Visibility, Visibility> for RustVisibilityParser {
    fn transform(&self, visibility: syn::Visibility, _config: &Config) -> Result<Visibility> {
        Ok(match visibility {
            syn::Visibility::Public(_) => Visibility::Public,
            _ => Visibility::Private
        })
    }
}

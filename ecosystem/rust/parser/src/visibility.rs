//! Visibility enumeration.

use crate::prelude::*;
use ligen::ir::Visibility;

#[derive(Default)]
pub struct VisibilityParser;

impl VisibilityParser {
    pub fn new() -> Self {
        Self
    }
}

impl Parser<syn::Visibility> for VisibilityParser {
    type Output = Visibility;
    fn parse(&self, visibility: syn::Visibility) -> Result<Self::Output> {
        Ok(match visibility {
            syn::Visibility::Public(_) => Self::Output::Public,
            _ => Self::Output::Private
        })
    }
}

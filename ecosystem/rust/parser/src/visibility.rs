//! Visibility enumeration.

use crate::prelude::*;
use ligen::ir::Visibility;

pub struct VisibilityParser;

impl Parser<syn::Visibility> for VisibilityParser {
    type Output = Visibility;
    fn parse(&self, visibility: syn::Visibility) -> Result<Self::Output> {
        Ok(match visibility {
            syn::Visibility::Public(_) => Self::Output::Public,
            _ => Self::Output::Private
        })
    }
}

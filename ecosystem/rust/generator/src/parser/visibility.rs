//! Visibility enumeration.

use crate::prelude::*;

use ligen_ir::Visibility;

impl From<SynVisibility> for Visibility {
    fn from(SynVisibility(visibility): SynVisibility) -> Self {
        match visibility {
            syn::Visibility::Public(_) => Self::Public,
            _ => Self::Private
        }
    }
}

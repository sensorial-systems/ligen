//! Visibility enumeration.

use crate::prelude::*;

use crate::Visibility;

impl From<SynVisibility> for Visibility {
    fn from(SynVisibility(visibility): SynVisibility) -> Self {
        match visibility {
            syn::Visibility::Public(_) => Self::Public,
            syn::Visibility::Crate(_) => Self::Crate,
            syn::Visibility::Restricted(_) => Self::Restricted,
            syn::Visibility::Inherited => Self::Inherited,
        }
    }
}

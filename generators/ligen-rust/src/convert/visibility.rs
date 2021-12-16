//! Visibility enumeration.

use crate::prelude::*;
use ligen_ir::Visibility;

impl From<syn::Visibility> for Visibility {
    fn from(visibility: syn::Visibility) -> Self {
        match visibility {
            syn::Visibility::Public(_) => Self::Public,
            syn::Visibility::Crate(_) => Self::Crate,
            syn::Visibility::Restricted(_) => Self::Restricted,
            syn::Visibility::Inherited => Self::Inherited,
        }
    }
}

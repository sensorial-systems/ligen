//! Macro attributes.

pub mod attributes;

use crate::MacroAttributes;
use crate::prelude::*;

impl ToTokens for MacroAttributes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let attributes = &self.attributes;
        tokens.append_all(quote! { #[#attributes] })
    }
}

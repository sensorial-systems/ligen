//! Macro attributes.

pub mod attributes;

use ligen_ir::MacroAttributes;
use crate::prelude::*;

impl ToTokens for MacroAttributes {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let attributes = &self.attributes.to_token_stream();
        tokens.append_all(quote! { #[#attributes] })
    }
}

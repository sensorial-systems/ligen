//! Macro attributes.

mod attributes;
pub use attributes::*;

use quote::{quote, ToTokens, TokenStreamExt};
use proc_macro2::TokenStream;
use shrinkwraprs::Shrinkwrap;

/// Macro attributes in the form of `#[attribute0, attribute1, ...]`.
#[derive(Shrinkwrap, Default, Debug, PartialEq, Clone)]
#[allow(missing_docs)]
pub struct MacroAttributes {
    pub attributes: Attributes
}

impl ToTokens for MacroAttributes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let attributes = &self.attributes;
        tokens.append_all(quote! { #[#attributes] })
    }
}

impl From<Attributes> for MacroAttributes {
    fn from(attributes: Attributes) -> Self {
        Self { attributes }
    }
}

impl From<Attribute> for MacroAttributes {
    fn from(attribute: Attribute) -> Self {
        let attributes = vec![attribute].into();
        Self { attributes }
    }
}
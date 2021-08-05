//! Procedural macro attributes.

use crate::ir::{Attributes, Attribute};
use quote::{quote, ToTokens, TokenStreamExt};
use proc_macro2::TokenStream;
use shrinkwraprs::Shrinkwrap;

/// Procedural macro attributes in the form of `#[attribute0, attribute1, ...]`.
#[derive(Shrinkwrap, Default, Debug, PartialEq, Clone)]
#[allow(missing_docs)]
pub struct ProceduralMacroAttributes {
    pub attributes: Attributes
}

impl ToTokens for ProceduralMacroAttributes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let attributes = &self.attributes;
        tokens.append_all(quote! { #[#attributes] })
    }
}

impl From<Attributes> for ProceduralMacroAttributes {
    fn from(attributes: Attributes) -> Self {
        Self { attributes }
    }
}

impl From<Attribute> for ProceduralMacroAttributes {
    fn from(attribute: Attribute) -> Self {
        let attributes = vec![attribute].into();
        Self { attributes }
    }
}
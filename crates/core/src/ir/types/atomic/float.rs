use quote::{quote, ToTokens, TokenStreamExt};
use proc_macro2::TokenStream;

#[derive(Debug, PartialEq, Copy, Clone)]
/// Float Enum
pub enum Float {
    /// f32 variant
    F32,
    /// f64 variant
    F64,
}

impl ToTokens for Float {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let typ = match self {
            Float::F32 => quote! {f32},
            Float::F64 => quote! {f64},
        };
        tokens.append_all(quote! {#typ})
    }
}

use quote::{quote, ToTokens, TokenStreamExt};
use proc_macro2::TokenStream;
use crate::ir::Identifier;

#[derive(Debug, PartialEq, Copy, Clone)]
/// Float Enum
pub enum Float {
    /// f32 variant
    F32,
    /// f64 variant
    F64,
}

impl std::fmt::Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let display = match self {
            Float::F32 => "f32",
            Float::F64 => "f64",
        };
        f.write_str(display)
    }
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

impl From<Float> for Identifier {
    fn from(float: Float) -> Self {
        match float {
            Float::F32 => "f32".into(),
            Float::F64 => "f64".into()
        }
    }
}
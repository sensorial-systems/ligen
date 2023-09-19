use crate::prelude::*;
use ligen_ir::Float;

impl ToTokens for Float {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let typ = match self {
            Float::F32 => quote! {f32},
            Float::F64 => quote! {f64},
        };
        tokens.append_all(quote! {#typ})
    }
}

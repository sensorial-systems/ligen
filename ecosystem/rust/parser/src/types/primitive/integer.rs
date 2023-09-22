use crate::prelude::*;
use ligen_ir::Integer;

impl ToTokens for Integer {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let type_ = match self {
            Integer::U8 => quote! {u8},
            Integer::U16 => quote! {u16},
            Integer::U32 => quote! {u32},
            Integer::U64 => quote! {u64},
            Integer::U128 => quote! {u128},
            Integer::USize => quote! {usize},
            Integer::I8 => quote! {i8},
            Integer::I16 => quote! {i16},
            Integer::I32 => quote! {i32},
            Integer::I64 => quote! {i64},
            Integer::I128 => quote! {i128},
            Integer::ISize => quote! {isize},
        };
        tokens.append_all(quote! {#type_})
    }
}

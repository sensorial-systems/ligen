use quote::{quote, ToTokens, TokenStreamExt};
use proc_macro2::TokenStream;

#[derive(Debug, Copy, Clone, PartialEq)]
/// Integer Enum
pub enum Integer {
    /// u8 variant
    U8,
    /// u16 variant
    U16,
    /// u32 variant
    U32,
    /// u64 variant
    U64,
    /// u128 variant
    U128,
    /// usize variant
    USize,
    /// i8 variant
    I8,
    /// i16 variant
    I16,
    /// i32 variant
    I32,
    /// i64 variant
    I64,
    /// i128 variant
    I128,
    /// isize variant
    ISize,
}

impl ToTokens for Integer {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let typ = match self {
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
        tokens.append_all(quote! {#typ})
    }
}
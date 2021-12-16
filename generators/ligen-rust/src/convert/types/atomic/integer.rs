use crate::prelude::*;
use crate::Identifier;
use ligen_ir::Integer;

impl std::fmt::Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let display = match self {
            Integer::U8    => "u8",
            Integer::U16   => "u16",
            Integer::U32   => "u32",
            Integer::U64   => "u64",
            Integer::U128  => "u128",
            Integer::USize => "usize",
            Integer::I8    => "i8",
            Integer::I16   => "i16",
            Integer::I32   => "i32",
            Integer::I64   => "i64",
            Integer::I128  => "i128",
            Integer::ISize => "isize",
        };
        f.write_str(display)
    }
}

impl ToTokens for Integer {
    fn to_tokens(&self, tokens: &mut TokenStream) {
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

impl From<Integer> for Identifier {
    fn from(integer: Integer) -> Self {
        match integer {
            Integer::U8    => "u8".into(),
            Integer::U16   => "u16".into(),
            Integer::U32   => "u32".into(),
            Integer::U64   => "u64".into(),
            Integer::U128  => "u128".into(),
            Integer::USize => "usize".into(),
            Integer::I8    => "i8".into(),
            Integer::I16   => "i16".into(),
            Integer::I32   => "i32".into(),
            Integer::I64   => "i64".into(),
            Integer::I128  => "i128".into(),
            Integer::ISize => "isize".into(),
        }
    }
}

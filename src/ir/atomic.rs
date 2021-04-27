use proc_macro2::Ident;

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

#[derive(Debug, PartialEq, Copy, Clone)]
/// Float Enum
pub enum Float {
    /// f32 variant
    F32,
    /// f64 variant
    F64,
}

#[derive(Debug, PartialEq, Clone, Copy)]
/// Atomic Enum
pub enum Atomic {
    /// Integer variant
    Integer(Integer),
    /// Float variant
    Float(Float),
    /// Boolean variant
    Boolean,
    /// Character variant
    Character,
}

impl From<Ident> for Atomic {
    fn from(ident: Ident) -> Self {
        match ident.to_string().as_str() {
            "u8" => Self::Integer(Integer::U8),
            "u16" => Self::Integer(Integer::U16),
            "u32" => Self::Integer(Integer::U32),
            "u64" => Self::Integer(Integer::U64),
            "u128" => Self::Integer(Integer::U128),
            "usize" => Self::Integer(Integer::USize),
            "i8" => Self::Integer(Integer::I8),
            "i16" => Self::Integer(Integer::I16),
            "i32" => Self::Integer(Integer::I32),
            "i64" => Self::Integer(Integer::I64),
            "i128" => Self::Integer(Integer::I128),
            "isize" => Self::Integer(Integer::ISize),
            "f32" => Self::Float(Float::F32),
            "f64" => Self::Float(Float::F64),
            "bool" => Self::Boolean,
            "char" => Self::Character,
            _ => panic!("Unknown Ident"),
        }
    }
}

impl From<syn::Path> for Atomic {
    fn from(path: syn::Path) -> Self {
        match path {
            syn::Path { segments, .. } => Self::from(segments[0].ident.clone()),
        }
    }
}

#[cfg(test)]
mod test {

    use super::{Atomic, Float, Integer};
    use quote::quote;
    use syn::parse_quote::parse;

    #[test]
    fn atomic_integer() {
        let vec: Vec<Atomic> = vec![
            quote! { u8 },
            quote! { u16 },
            quote! { u32 },
            quote! { u64 },
            quote! { u128 },
            quote! { usize },
            quote! { i8 },
            quote! { i16 },
            quote! { i32 },
            quote! { i64 },
            quote! { i128 },
            quote! { isize },
        ]
        .into_iter()
        .map(|x| parse::<syn::Ident>(x).into())
        .collect();
        let expected: Vec<Integer> = vec![
            Integer::U8,
            Integer::U16,
            Integer::U32,
            Integer::U64,
            Integer::U128,
            Integer::USize,
            Integer::I8,
            Integer::I16,
            Integer::I32,
            Integer::I64,
            Integer::I128,
            Integer::ISize,
        ]
        .into_iter()
        .collect();

        let mut iter = vec.iter().zip(expected.iter());

        while let Some((Atomic::Integer(value), expected_value)) = iter.next() {
            assert_eq!(value, expected_value);
        }
    }

    #[test]
    fn atomic_float() {
        let vec: Vec<Atomic> = vec![quote! { f32 }, quote! { f64 }]
            .into_iter()
            .map(|x| parse::<syn::Ident>(x).into())
            .collect();
        let expected: Vec<Float> = vec![Float::F32, Float::F64].into_iter().collect();

        let mut iter = vec.iter().zip(expected.iter());

        while let Some((Atomic::Float(value), expected_value)) = iter.next() {
            assert_eq!(value, expected_value);
        }
    }

    #[test]
    fn atomic_boolean() {
        assert_eq!(Atomic::Boolean, parse::<syn::Ident>(quote! {bool}).into());
    }

    #[test]
    fn atomic_character() {
        assert_eq!(Atomic::Character, parse::<syn::Ident>(quote! {char}).into());
    }
}

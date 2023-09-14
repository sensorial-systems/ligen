use crate::prelude::*;
use crate::Identifier;

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

impl Integer {
    /// Check if the `Integer` is unsigned.
    pub fn is_unsigned(&self) -> bool {
        match self {
            Self::U8 | Self::U16 | Self::U32 | Self::U64 | Self::U128 | Self::USize => true,
            _ => false
        }
    }
}

impl From<Integer> for Identifier {
    fn from(from: Integer) -> Self {
        format!("{:#?}", from).into()
    }
}

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

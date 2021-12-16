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

impl From<Integer> for Identifier {
    fn from(from: Integer) -> Self {
        format!("{:#?}", from).into()
    }
}

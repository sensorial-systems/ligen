use crate::prelude::*;
use ligen_ir::Identifier;
use ligen_ir::Integer;
use crate::traits::AsRust;

impl AsRust for Integer {
    fn as_rust(&self) -> String {
        match self {
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
        }.into()
    }
}

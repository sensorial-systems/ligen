use crate::prelude::*;
use ligen_ir::Float;
use crate::traits::AsRust;

impl AsRust for Float {
    fn as_rust(&self) -> String {
        match self {
            Float::F32 => "f32",
            Float::F64 => "f64",
        }.into()
    }
}

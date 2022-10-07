use crate::prelude::*;
use crate::Identifier;

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
/// Float Enum
pub enum Float {
    /// f32 variant
    F32,
    /// f64 variant
    F64,
}

impl From<Float> for Identifier {
    fn from(from: Float) -> Self {
        format!("{:#?}", from).into()
    }
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
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

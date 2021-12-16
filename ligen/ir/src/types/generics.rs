use crate::Type;
use crate::prelude::*;

/// Generic arguments list.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
pub struct Generics {
    /// Generic types.
    pub types: Vec<Type>
}

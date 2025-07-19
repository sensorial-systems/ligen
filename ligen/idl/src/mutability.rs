//! Mutability.

use crate::prelude::*;

/// Mutability.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, EnumIter, JsonSchema)]
#[allow(missing_docs)]
pub enum Mutability {
    Constant,
    Mutable
}

impl Default for Mutability {
    fn default() -> Self {
        Self::Mutable
    }
}

//! Mutability.

use crate::prelude::*;

/// Mutability.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, EnumIter, JsonSchema)]
#[allow(missing_docs)]
#[derive(Default)]
pub enum Mutability {
    Constant,
    #[default]
    Mutable
}


//! Mutability.

use serde::{Serialize, Deserialize};
use crate::prelude::*;

/// Mutability.
#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone, Serialize, Deserialize, EnumIter)]
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

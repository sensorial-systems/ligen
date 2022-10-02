//! Mutability.

use serde::{Serialize, Deserialize};

/// Mutability.
#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum Mutability {
    Constant,
    Mutable
}

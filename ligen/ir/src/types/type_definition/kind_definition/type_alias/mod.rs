//! Type alias module.
use crate::{prelude::*, Type};

#[cfg(any(test, feature = "mocks"))]
pub mod mock;

/// Type alias representation.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeAlias {
    pub type_: Type
}

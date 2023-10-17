//! Import representation.

#[cfg(any(test, feature = "mocks"))]
pub mod mock;

use crate::prelude::*;
use crate::{Path, Attributes, Visibility, Identifier};

/// Import representation.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Import {
    /// Attributes.
    pub attributes: Attributes,
    /// Visibility.
    pub visibility: Visibility,
    /// Path of the imported object.
    pub path: Path,
    /// Optional renaming.
    pub renaming: Option<Identifier>
}

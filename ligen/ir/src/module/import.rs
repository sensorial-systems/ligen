//! Import representation.

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

/// Multiple imports.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Imports(pub Vec<Import>);

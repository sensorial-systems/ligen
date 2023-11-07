//! Library representation.

pub mod metadata;
pub use metadata::*;

use crate::Identifier;
use crate::Module;
use crate::prelude::*;

/// Library representation.
#[allow(missing_docs)]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Library {
    pub identifier: Identifier,
    pub metadata: Metadata,
    pub root_module: Module,
}

impl Library {
    /// Save library to file.
    pub fn save(&self, path: impl AsRef<std::path::Path>) -> Result<()> {
        let contents = serde_json::to_string_pretty(self)?;
        std::fs::write(path, contents)?;
        Ok(())
    }

    /// Load library from file.
    pub fn load(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&json)?)
    }
}

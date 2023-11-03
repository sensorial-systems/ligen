// FIXME: Look for mentions to "library" and rename it to "library".
//! Library representation.

use crate::Module;
use crate::prelude::*;
use crate::conventions::naming::NamingConvention;

/// Library representation.
#[allow(missing_docs)]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Library {
    pub name: NamingConvention,
    pub root_module: Module,
}

impl Library {
    /// Get the library name.
    pub fn name(&self) -> &NamingConvention {
        &self.name
    }

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

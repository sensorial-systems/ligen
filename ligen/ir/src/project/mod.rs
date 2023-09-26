//! Project representation.

use crate::Module;
use crate::prelude::*;
use crate::conventions::naming::NamingConvention;

/// Project representation.
#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Project {
    pub directory: std::path::PathBuf,
    pub name: NamingConvention,
    pub root_module: Module,
}

impl Project {
    /// Project directory path.
    pub fn directory(&self) -> &std::path::Path {
        self.directory.as_path()
    }
    /// Get the project name.
    pub fn name(&self) -> &NamingConvention {
        &self.name
    }

    /// Save project to file.
    pub fn save(&self, path: impl AsRef<std::path::Path>) -> Result<()> {
        let contents = serde_json::to_string_pretty(self)?;
        std::fs::write(path, contents)?;
        Ok(())
    }

    /// Load project from file.
    pub fn load(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&json)?)
    }
}

//! BuildType definition module.

use crate::prelude::*;
use serde::{Deserialize, Serialize};

/// Release or Debug.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[allow(missing_docs)]
pub enum BuildType {
    Release,
    Debug,
}

impl BuildType {
    /// Gets BuildType from `PROFILE` environment variable.
    pub fn from_env() -> Result<Self> {
        let profile = std::env::var("PROFILE")?;
        let build_type = if profile == "release" {
            BuildType::Release
        } else {
            BuildType::Debug
        };
        Ok(build_type)
    }
}

impl Default for BuildType {
    fn default() -> Self {
        Self::Debug
    }
}

impl ToString for BuildType {
    fn to_string(&self) -> String {
        format!("{:#?}", self)
    }
}

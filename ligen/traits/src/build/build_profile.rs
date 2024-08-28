//! BuildType definition module.

use crate::prelude::*;

/// Release or Debug. Defaults to the current's Cargo build process build profile.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[allow(missing_docs)]
pub enum BuildProfile {
    Release,
    Debug,
}

/// Build profile of the current Cargo build operation.
#[cfg(debug_assertions)]
const BUILD_PROFILE: BuildProfile = BuildProfile::Debug;

/// Build profile of the current Cargo build operation.
#[cfg(not(debug_assertions))]
const BUILD_PROFILE: BuildProfile = BuildProfile::Release;

impl Default for BuildProfile {
    fn default() -> Self {
        BUILD_PROFILE
    }
}

impl std::fmt::Display for BuildProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildProfile::Release => write!(f, "Release"),
            BuildProfile::Debug => write!(f, "Debug"),
        }
    }
}

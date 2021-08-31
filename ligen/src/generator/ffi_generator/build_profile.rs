//! BuildType definition module.

use serde::{Deserialize, Serialize};

/// Release or Debug.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[allow(missing_docs)]
pub enum BuildProfile {
    Release,
    Debug,
}

/// Build profile of the current Cargo build operation.
#[cfg(debug_assertions)]
pub const BUILD_PROFILE: BuildProfile = BuildProfile::Debug;

/// Build profile of the current Cargo build operation.
#[cfg(not(debug_assertions))]
pub const BUILD_PROFILE: BuildProfile = BuildProfile::Release;

impl Default for BuildProfile {
    fn default() -> Self {
        Self::Debug
    }
}

impl ToString for BuildProfile {
    fn to_string(&self) -> String {
        format!("{:#?}", self)
    }
}

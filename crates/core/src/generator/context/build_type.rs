//! BuildType definition module.

use serde::{Deserialize, Serialize};

/// Release or Debug.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum BuildType {
    #[allow(missing_docs)]
    Release,
    #[allow(missing_docs)]
    Debug,
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

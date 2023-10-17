use crate::prelude::*;

/// Synchrony structure.
#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize, EnumIter)]
pub enum Synchrony {
    Synchronous,
    Asynchronous
}

impl Default for Synchrony {
    fn default() -> Self {
        Self::Synchronous
    }
}

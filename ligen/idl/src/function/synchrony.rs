use std::fmt::Display;

use crate::prelude::*;

/// Synchrony structure.
#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize, EnumIter, JsonSchema)]
#[derive(Default)]
pub enum Synchrony {
    #[default]
    Synchronous,
    Asynchronous
}


impl Display for Synchrony {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Synchrony::Synchronous => write!(f, "Synchronous"),
            Synchrony::Asynchronous => write!(f, "Asynchronous")
        }
    }
}
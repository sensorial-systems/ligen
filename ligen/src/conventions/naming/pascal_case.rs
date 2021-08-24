//! PascalCase.

use derive_more::Display;
use serde::{Serialize, Deserialize};

/// PascalCase.
#[derive(Debug, Default, Clone, Serialize, Deserialize, Display, PartialEq, Eq)]
#[display(fmt = "{}", _0)]
pub struct PascalCase(String);

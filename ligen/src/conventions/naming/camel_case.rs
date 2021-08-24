//! camelCase.

use derive_more::Display;
use serde::{Serialize, Deserialize};

/// camelCase.
#[derive(Debug, Default, Clone, Serialize, Deserialize, Display, PartialEq, Eq)]
#[display(fmt = "{}", _0)]
pub struct CamelCase(String);

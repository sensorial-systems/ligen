use std::fmt::{Display, Formatter};

use crate::prelude::*;
use crate::{Path, Literal};

/// Attribute enumeration.
#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Named {
    /// Path of the attribute.
    pub path: Path,
    /// Literal of the attribute.
    pub literal: Literal,
}

impl Named {
    /// Creates a new named attribute.
    pub fn new<I: Into<Path>, L: Into<Literal>>(path: I, literal: L) -> Self {
        let path = path.into();
        let literal = literal.into();
        Self { path, literal }
    }
}

impl Display for Named {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.path, self.literal)
    }
}
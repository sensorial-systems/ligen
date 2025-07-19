use std::fmt::{Display, Formatter};

use crate::prelude::*;

use crate::{Path, Identifier, Attributes};

/// Attributes group.
#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Group {
    /// Path of the group.
    pub path: Path,
    /// Attributes of the group.
    pub attributes: Attributes,
}

impl Group {
    pub fn new<I: Into<Path>, A: Into<Attributes>>(path: I, attributes: A) -> Self {
        let path = path.into();
        let attributes = attributes.into();
        Self { path, attributes }
    }
}

impl From<&str> for Group {
    fn from(value: &str) -> Self {
        Self { path: value.into(), ..Default::default() }
    }
}

impl From<Path> for Group {
    fn from(path: Path) -> Self {
        Self { path, ..Default::default() }
    }
}

impl From<Identifier> for Group {
    fn from(identifier: Identifier) -> Self {
        Path::from(identifier).into()
    }
}

impl Display for Group {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.path, self.attributes)
    }
}
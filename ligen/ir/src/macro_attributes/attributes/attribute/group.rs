use std::fmt::{Display, Formatter};

use crate::prelude::*;

use crate::{Identifier, Attributes};

/// Attributes group.
#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Group {
    /// Identifier of the group.
    pub identifier: Identifier,
    /// Attributes of the group.
    pub attributes: Attributes,
}

impl Group {
    pub fn new<I: Into<Identifier>, A: Into<Attributes>>(identifier: I, attributes: A) -> Self {
        let identifier = identifier.into();
        let attributes = attributes.into();
        Self { identifier, attributes }
    }
}

impl From<&str> for Group {
    fn from(value: &str) -> Self {
        Self { identifier: value.into(), ..Default::default() }
    }
}

impl From<Identifier> for Group {
    fn from(identifier: Identifier) -> Self {
        Self { identifier, ..Default::default() }
    }
}

impl Display for Group {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.identifier, self.attributes)
    }
}
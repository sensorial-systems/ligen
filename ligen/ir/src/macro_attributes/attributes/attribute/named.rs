use std::fmt::{Display, Formatter};

use crate::prelude::*;
use crate::{Identifier, Literal};

/// Attribute enumeration.
#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Named {
    /// Identifier of the attribute.
    pub identifier: Identifier,
    /// Literal of the attribute.
    pub literal: Literal,
}

impl Named {
    /// Creates a new named attribute.
    pub fn new<I: Into<Identifier>, L: Into<Literal>>(identifier: I, literal: L) -> Self {
        let identifier = identifier.into();
        let literal = literal.into();
        Self { identifier, literal }
    }
}

impl Display for Named {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.identifier, self.literal)
    }
}
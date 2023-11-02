use std::fmt::Display;

use crate::{prelude::*, Identifier, Generics};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PathSegment {
    pub identifier: Identifier,
    pub generics: Generics
}

impl From<&str> for PathSegment {
    fn from(value: &str) -> Self {
        Identifier::from(value).into()
    }
}

impl From<Identifier> for PathSegment {
    fn from(identifier: Identifier) -> Self {
        let generics = Default::default();
        Self { identifier, generics }
    }
}

impl Display for PathSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.identifier, self.generics)
    }
}
use std::fmt::Display;

use crate::{prelude::*, Identifier, Generics, Type};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PathSegment {
    pub identifier: Identifier,
    pub generics: Generics
}

impl PathSegment {
    pub fn new(identifier: impl Into<Identifier>, generics: impl Into<Generics>) -> Self {
        let identifier = identifier.into();
        let generics = generics.into();
        Self { identifier, generics }
    }    
}

impl From<&str> for PathSegment {
    fn from(value: &str) -> Self {
        Identifier::from(value).into()
    }
}

impl From<String> for PathSegment {
    fn from(value: String) -> Self {
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

// FIXME: Remove this.
// impl IntoIterTypeMut<Type> for PathSegment {
//     fn type_iterator(&mut self) -> TypeIterMut<'_, Type> {
//         self.generics.type_iterator()
//     }
// }
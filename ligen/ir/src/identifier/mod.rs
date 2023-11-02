#[cfg(any(test, feature = "mocks"))]
pub mod mock;

use crate::path::PathSegment;
use crate::prelude::*;

use crate::conventions::naming::SnakeCase;

/// Identifier structure
#[derive(Clone, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Display, Serialize, Deserialize)]
#[display(fmt = "{}", name)]
pub struct Identifier {
    /// Name field of Identifier
    pub name: String,
}

impl Identifier {
    /// Create a new Identifier
    pub fn new<S: AsRef<str>>(name: S) -> Self {
        let name = String::from(name.as_ref());
        Self { name }
    }
}

impl From<SnakeCase> for Identifier {
    fn from(snake_case: SnakeCase) -> Self {
        snake_case.to_string().into()
    }
}

impl From<&str> for Identifier {
    fn from(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}

impl From<String> for Identifier {
    fn from(name: String) -> Self {
        name.as_str().into()
    }
}

impl From<PathSegment> for Identifier {
    fn from(value: PathSegment) -> Self {
        value.identifier
    }
}

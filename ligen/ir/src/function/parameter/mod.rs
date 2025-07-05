//! Function parameter.

#[cfg(any(test, feature = "mocks"))]
pub mod mock;

use std::fmt::{Display, Formatter};

use crate::{prelude::*, Literal};
use crate::{Identifier, Type, Attributes, Mutability};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize, JsonSchema)]
/// Parameter representation.
pub struct Parameter {
    /// Attributes.
    pub attributes: Attributes,
    /// Identifier.
    pub identifier: Identifier,
    /// Type.
    pub type_: Type,
    /// Default value.
    pub default_value: Option<Literal>,
}

impl Parameter {
    pub fn new(identifier: impl Into<Identifier>, type_: impl Into<Type>) -> Self {
        let attributes = Attributes::default();
        let identifier = identifier.into();
        let type_ = type_.into();
        let default_value = Default::default();
        Self { attributes, identifier, type_, default_value }
    }
}

impl Parameter {
    /// Get parameter mutability.
    pub fn mutability(&self) -> Mutability {
        if self.type_.is_mutable_reference() {
            Mutability::Mutable
        } else {
            Mutability::Constant
        }
    }
}

impl Display for Parameter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let attributes = if self.attributes.is_empty() { "".into() } else { format!(" {}", self.attributes) };
        f.write_str(&format!("{}: {}{}", self.identifier, self.type_, attributes))
    }
}

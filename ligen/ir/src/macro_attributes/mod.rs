//! Macro attributes.

pub mod attributes;
pub use attributes::*;

use crate::prelude::*;

/// Macro attributes in the form of `#[attribute0, attribute1, ...]`.
#[derive(Shrinkwrap, Default, Debug, PartialEq, Clone)]
#[allow(missing_docs)]
pub struct MacroAttributes {
    pub attributes: Attributes
}

impl From<Attributes> for MacroAttributes {
    fn from(attributes: Attributes) -> Self {
        Self { attributes }
    }
}

impl From<Attribute> for MacroAttributes {
    fn from(attribute: Attribute) -> Self {
        let attributes = vec![attribute].into();
        Self { attributes }
    }
}
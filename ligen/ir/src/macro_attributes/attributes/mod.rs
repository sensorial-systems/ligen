mod attribute;
pub use attribute::*;

use crate::{Identifier, Path};
use crate::Literal;
use crate::prelude::*;

#[derive(Shrinkwrap, Default, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[shrinkwrap(mutable)]
/// Attributes representation.
pub struct Attributes {
    /// attributes field
    pub attributes: Vec<Attribute>,
}

impl Attributes {
    /// Get the group identified by `path`.
    pub fn get_subgroup<P: Into<Path>>(&self, path: P) -> Option<&Attributes> {
        let path = path.into();
        let mut group = self;
        for segment in path.segments {
            if let Some(new_group) = group.get_group(segment) {
                group = new_group
            } else {
                return None;
            }
        }
        Some(group)
    }

    /// Get a literal from the `path`.
    pub fn get_literal_from_path<P: Into<Path>>(&self, path: P) -> Option<&Literal> {
        let mut path = path.into();
        path
            .pop_back()
            .and_then(|last| {
                self
                    .get_subgroup(path.segments)
                    .and_then(|group| group.get_named(last))
            })
    }

    /// Get the group identified by `name`.
    pub fn get_group<I: Into<Identifier>>(&self, name: I) -> Option<&Attributes> {
        let name = name.into();
        self
            .attributes
            .iter()
            .find_map(|attribute| {
                if let Attribute::Group(identifier, attributes) = attribute {
                    if *identifier == name {
                        Some(attributes)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
    }

    /// Get named attribute e.g.: name = "literal"
    pub fn get_named<I: Into<Identifier>>(&self, name: I) -> Option<&Literal> {
        let name = name.into();
        self
            .attributes
            .iter()
            .find_map(|attribute| {
                if let Attribute::Named(identifier, literal) = attribute {
                    if *identifier == name {
                        Some(literal)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
    }

    /// Check if `Attributes` contains the specified `attribute`.
    pub fn contains(&self, attribute: &Attribute) -> bool {
        self
            .attributes
            .iter()
            .find(|inner_attribute| **inner_attribute == *attribute)
            .is_some()
    }
}

impl From<Vec<Attribute>> for Attributes {
    fn from(attributes: Vec<Attribute>) -> Self {
        Self { attributes }
    }
}

impl From<Attribute> for Attributes {
    fn from(attribute: Attribute) -> Self {
        let attributes = vec![attribute];
        Self { attributes }
    }
}

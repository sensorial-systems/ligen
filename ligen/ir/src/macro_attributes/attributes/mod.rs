pub mod attribute;

#[cfg(any(test, feature = "mocks"))]
pub mod mock;

use std::fmt::{Display, Formatter};
pub use attribute::*;

use crate::Path;
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
    /// Get documentation from the attributes.
    pub fn get_documentation(&self) -> Vec<String> {
        self
            .iter()
            .filter_map(|attr| attr.as_named())
            .filter(|named| named.path.segments.last().is_some())
            .filter(|named| named.path.segments.last().unwrap().identifier == "doc")
            .filter_map(|named| named.literal.as_string())
            .map(|doc| doc.trim().to_string())
            .collect::<Vec<_>>()
    }

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
    pub fn get_group<I: Into<Path>>(&self, name: I) -> Option<&Attributes> {
        let name = name.into();
        self
            .attributes
            .iter()
            .find_map(|attribute| {
                if let Attribute::Group(group) = attribute {
                    if group.path == name {
                        Some(&group.attributes)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
    }

    /// Get named attribute e.g.: name = "literal"
    pub fn get_named<I: Into<Path>>(&self, name: I) -> Option<&Literal> {
        let name = name.into();
        self
            .attributes
            .iter()
            .find_map(|attribute| {
                if let Attribute::Named(named) = attribute {
                    if named.path == name {
                        Some(&named.literal)
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
            .any(|inner_attribute| *inner_attribute == *attribute)
    }

    /// Check if the attributes list has an ignore attribute.
    pub fn has_ignore_attribute(&self) -> bool {
        self.contains(&Attribute::Group(Group::new("ligen", Group::from("ignore"))))
    }
}

impl From<Group> for Attributes {
    fn from(group: Group) -> Self {
        Self { attributes: vec![group.into()] }
    }
}

impl From<Named> for Attributes {
    fn from(named: Named) -> Self {
        Self { attributes: vec![named.into()] }
    }
}

impl<L: Into<Literal>> From<L> for Attributes {
    fn from(literal: L) -> Self {
        Self { attributes: vec![literal.into().into()] }
    }
}

impl<A: Into<Attribute>> From<Vec<A>> for Attributes {
    fn from(attributes: Vec<A>) -> Self {
        let attributes = attributes
            .into_iter()
            .map(|attribute| attribute.into())
            .collect();
        Self { attributes }
    }
}

impl From<Attribute> for Attributes {
    fn from(attribute: Attribute) -> Self {
        let attributes = vec![attribute];
        Self { attributes }
    }
}

impl Display for Attributes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.attributes.iter().map(|attribute| attribute.to_string()).collect::<Vec<_>>().join(",").as_str())
    }
}

#[cfg(test)]
mod test {
    use crate::macro_attributes::attributes::mock;
    use crate::*;

    #[test]
    fn get_literals() {
        let attributes = mock::parse_literals();
        assert_eq!(attributes.get_literal_from_path(vec!["c", "int"]), Some(&Literal::String("sized".into())));
        assert_eq!(attributes.get_literal_from_path(vec!["c", "marshal_as", "name"]), Some(&Literal::String("hello".into())));
        assert_eq!(attributes.get_literal_from_path(vec!["c", "marshal_as", "uuid"]), Some(&Literal::Integer(5)));
    }
}
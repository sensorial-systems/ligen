// TODO: Is this still used? We replaced it with String (so different types of version strings can be used).

use crate::prelude::*;

#[derive(Shrinkwrap, Default, Debug, Display, Clone, Serialize, Deserialize, PartialEq)]
#[shrinkwrap(mutable)]
pub struct VersionRequirement(pub String);

impl From<&str> for VersionRequirement {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

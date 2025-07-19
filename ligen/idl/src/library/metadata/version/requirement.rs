// TODO: Is this still used? We replaced it with String (so different types of version strings can be used).

use crate::prelude::*;

#[derive(Shrinkwrap, Default, Debug, Display, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
#[shrinkwrap(mutable)]
pub struct VersionRequirement(pub String);

impl From<String> for VersionRequirement {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for VersionRequirement {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}

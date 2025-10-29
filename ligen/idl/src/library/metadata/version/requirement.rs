// TODO: Is this still used? We replaced it with String (so different types of version strings can be used).

use crate::prelude::*;

#[derive(Shrinkwrap, Default, Debug, Display, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
#[shrinkwrap(mutable)]
pub struct VersionRequirement(pub String);

impl VersionRequirement {
    pub fn any() -> Self {
        Self("*".to_string())
    }

    pub fn exact(version: &str) -> Self {
        Self(version.to_string())
    }

    pub fn range(start: &str, end: &str) -> Self {
        Self(format!("{start}..{end}"))
    }

    pub fn greater_than(version: &str) -> Self {
        Self(format!("> {version}"))
    }

    pub fn greater_than_equal(version: &str) -> Self {
        Self(format!(">= {version}"))
    }

    pub fn less_than(version: &str) -> Self {
        Self(format!("< {version}"))
    }

    pub fn less_than_equal(version: &str) -> Self {
        Self(format!("<= {version}"))
    }
}

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

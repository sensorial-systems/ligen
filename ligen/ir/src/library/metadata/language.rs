use crate::{prelude::*, VersionRequirement};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct Language {
    pub name: String,
    pub requirement: Option<VersionRequirement>,
}

impl Language {
    pub fn new(name: impl Into<String>, requirement: Option<VersionRequirement>) -> Self {
        let name = name.into();
        Self { name, requirement }
    }
}


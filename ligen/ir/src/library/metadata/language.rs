use crate::{prelude::*, VersionRequirement};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Language {
    pub name: String,
    pub requirement: VersionRequirement,
}
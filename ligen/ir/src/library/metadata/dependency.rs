use crate::{prelude::*, VersionRequirement};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Dependency {
    pub identifier: String,
    pub requirement: VersionRequirement,
}

impl TryFrom<&str> for Dependency {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self> {
        let mut parts = value.split(' ');
        let identifier = parts.next().ok_or("Failed to get identifier.")?.to_string();
        let requirement = parts.next().ok_or("Failed to get requirement.")?;
        let requirement = VersionRequirement::try_from(requirement)?;
        Ok(Self { identifier, requirement })
    }
}
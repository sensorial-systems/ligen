use crate::{prelude::*, VersionRequirement, Identifier};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Dependency {
    pub identifier: Identifier,
    pub requirement: VersionRequirement,
}

impl TryFrom<&str> for Dependency {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self> {
        let value = value.trim().split(';').next().unwrap_or(value);
        let mut parts = value.split(' ');
        let identifier = Identifier::from(parts.next().ok_or("Failed to get identifier.")?);
        let mut requirement = None;
        for part in parts {
            if let Ok(parsed) = VersionRequirement::try_from(part) {
                requirement = Some(parsed);
                break;
            }
        }
        let requirement = requirement.unwrap_or_default();
        Ok(Self { identifier, requirement })
    }
}
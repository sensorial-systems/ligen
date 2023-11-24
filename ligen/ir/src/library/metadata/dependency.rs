use crate::{prelude::*, VersionRequirement};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Dependency {
    pub identifier: String,
    pub requirement: VersionRequirement,
}

impl TryFrom<&str> for Dependency {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self> {
        let value = value.trim().split(';').next().unwrap_or(value);
        let mut parts = value.split(' ');
        let identifier = parts.next().ok_or("Failed to get identifier.")?.to_string();
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
use crate::prelude::*;

#[derive(Shrinkwrap, Debug, Display, Clone, Serialize, Deserialize, PartialEq)]
#[shrinkwrap(mutable)]
pub struct VersionRequirement(pub semver::VersionReq);

impl TryFrom<&str> for VersionRequirement {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self> {
        let value = value.replace("==", "=");
        let version =
            semver::VersionReq::parse(&value)
                .map_err(|e| Error::Message(format!("Failed to parse version requirement: {}, Reason: {}", value, e)))?;
        let version = Self(version);
        Ok(version)
    }
}

impl Default for VersionRequirement {
    fn default() -> Self {
        Self(semver::VersionReq::parse("*").unwrap())
    }
}

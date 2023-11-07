use crate::prelude::*;

pub mod requirement;
pub use requirement::*;

#[derive(Shrinkwrap, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[shrinkwrap(mutable)]
pub struct Version(pub semver::Version);

impl Default for Version {
    fn default() -> Self {
        Self(semver::Version::new(0, 1, 0))
    }
}

impl TryFrom<&str> for Version {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self> {
        let version =
            semver::Version::parse(value)
                .map_err(|e| Error::Message(format!("Failed to parse version: {}, Reason: {}", value, e)))?;
        let version = Self(version);
        Ok(version)
    }
}
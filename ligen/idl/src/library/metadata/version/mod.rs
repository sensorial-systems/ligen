use crate::prelude::*;

pub mod requirement;
pub use requirement::*;

#[derive(Shrinkwrap, Debug, Display, Clone, Serialize, Deserialize, PartialEq)]
#[shrinkwrap(mutable)]
pub struct Version(pub semver::Version);

impl JsonSchema for Version {
    fn schema_name() -> String {
        "Version".to_string()
    }

    fn json_schema(generator: &mut schemars::r#gen::SchemaGenerator) -> schemars::schema::Schema {
        generator.subschema_for::<String>()
    }
}

impl Default for Version {
    fn default() -> Self {
        Self(semver::Version::new(0, 1, 0))
    }
}

impl Version {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self(semver::Version::new(major, minor, patch))
    }
}

impl TryFrom<String> for Version {
    type Error = Error;
    fn try_from(value: String) -> Result<Self> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<&str> for Version {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self> {
        let version =
            semver::Version::parse(value)
                .map_err(|e| Error::Message(format!("Failed to parse version: {value}, Reason: {e:?}")))?;
        let version = Self(version);
        Ok(version)
    }
}
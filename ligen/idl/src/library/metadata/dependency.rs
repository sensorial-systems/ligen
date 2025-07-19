use regex::Regex;

use crate::{prelude::*, VersionRequirement, Identifier};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct Dependency {
    pub identifier: Identifier,
    pub requirement: VersionRequirement,
    pub features: Vec<Identifier>,
}

// TODO: This has become Python-centric. We need to move it to a parser logic.
impl TryFrom<&str> for Dependency {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self> {
        let value = value.trim();
        let mut values = value.split(';');
        let value = values.next().unwrap_or(value);
        let feature = values.next().and_then(|v| {
            Regex::new(r"extra\s*==\s*'([^']*)'")
                .unwrap()
                .captures(v)
                .map(|c| Identifier::from(c.get(1).unwrap().as_str()))
        });
        let features = feature.into_iter().collect::<Vec<_>>();
        let regex = regex::Regex::new(r"(==|>=|<=|>|<)").map_err(|e| format!("{e:?}"))?;
        let mut parts = regex.split(value);
        let identifier = Identifier::from(parts.next().ok_or("Failed to get identifier.")?);
        let rest = parts.collect::<Vec<_>>().join(" ");
        let requirement = VersionRequirement::from(rest.as_str());
        Ok(Self { identifier, requirement, features })
    }
}
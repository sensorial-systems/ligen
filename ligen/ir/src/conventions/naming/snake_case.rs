//! snake_case.

use crate::prelude::*;
use super::{NamingConvention, KebabCase};

/// snake_case.
#[derive(Debug, Default, Clone, Serialize, Deserialize, Display, PartialEq, Eq)]
#[display(fmt = "{}", _0)]
pub struct SnakeCase(String);

impl TryFrom<NamingConvention> for SnakeCase {
    type Error = Error;
    fn try_from(name: NamingConvention) -> Result<Self> {
        match name {
            NamingConvention::KebabCase(name) => Ok(name.into()),
            _ => todo!("Implement other naming conventions.")
        }
    }
}

impl From<KebabCase> for SnakeCase {
    fn from(name: KebabCase) -> Self {
        Self(name.to_string().replace("-" ,"_"))
    }
}

impl TryFrom<&str> for SnakeCase {
    type Error = Error;
    fn try_from(naming: &str) -> Result<Self> {
        Ok(Self(naming.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_kebab_case() -> Result<()> {
        let kebab_case = KebabCase::try_from("naming-convention").expect("Not in snake case.");
        let snake_case = SnakeCase::from(kebab_case);
        assert_eq!(snake_case.to_string(), "naming_convention");
        Ok(())
    }
}

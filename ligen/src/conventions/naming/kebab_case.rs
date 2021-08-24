//! kebab-case.

use crate::prelude::*;
use super::SnakeCase;
use derive_more::Display;
use serde::{Serialize, Deserialize};

/// kebab-case.
#[derive(Debug, Default, Clone, Serialize, Deserialize, Display, PartialEq, Eq)]
#[display(fmt = "{}", _0)]
pub struct KebabCase(String);

impl From<SnakeCase> for KebabCase {
    fn from(name: SnakeCase) -> Self {
        Self(name.to_string().replace("_" ,"-"))
    }
}

impl TryFrom<&str> for KebabCase {
    type Error = Error;
    fn try_from(naming: &str) -> Result<Self> {
        Ok(Self(naming.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn from_snake_case() {
        let snake_case = SnakeCase::try_from("naming_convention").expect("Not in snake case.");
        let kebab_case = KebabCase::from(snake_case);
        assert_eq!(kebab_case.to_string(), "naming-convention");
    }
}
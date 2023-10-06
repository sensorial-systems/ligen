//! camelCase.

use crate::prelude::*;
use super::{NamingConvention, KebabCase};

/// camelCase.
#[derive(Debug, Default, Clone, Serialize, Deserialize, Display, PartialEq, Eq)]
#[display(fmt = "{}", _0)]
pub struct CamelCase(String);

impl TryFrom<&str> for CamelCase {
    type Error = Error;
    fn try_from(naming: &str) -> Result<Self> {
        Ok(Self(naming.to_string()))
    }
}

impl From<NamingConvention> for CamelCase {
    fn from(name: NamingConvention) -> Self {
        match name {
            NamingConvention::KebabCase(name) => name.into(),
            _ => todo!("Implement other naming conventions.")
        }
    }
}

impl From<KebabCase> for CamelCase {
    fn from(name: KebabCase) -> Self {
        let name = name.to_string();
        let characters = name.chars();
        let mut first_letter = false;
        let mut camel_name = String::new();
        // FIXME: How can we clean up this code? :|
        for character in characters {
            if first_letter {
                camel_name.push(character.to_ascii_uppercase());
                first_letter = false;
            } else if character == '-' {
                first_letter = true;
            } else {
                camel_name.push(character);
            }
        }
        Self(camel_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_kebab_case() {
        let kebab_case = KebabCase::try_from("naming-convention").expect("Not in snake case.");
        let camel_case = CamelCase::from(kebab_case);
        assert_eq!(camel_case.to_string(), "namingConvention");
    }
}

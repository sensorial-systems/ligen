//! PascalCase.

use crate::conventions::naming::{NamingConvention, KebabCase};
use crate::prelude::*;

/// PascalCase.
#[derive(Debug, Default, Clone, Serialize, Deserialize, Display, PartialEq, Eq)]
#[display(fmt = "{}", _0)]
pub struct PascalCase(String);

impl TryFrom<&str> for PascalCase {
    type Error = Error;
    fn try_from(naming: &str) -> Result<Self> {
        Ok(Self(naming.to_string()))
    }
}

impl From<NamingConvention> for PascalCase {
    fn from(name: NamingConvention) -> Self {
        match name {
            NamingConvention::KebabCase(name) => name.into(),
            _ => todo!("Implement other naming conventions.")
        }
    }
}

impl From<KebabCase> for PascalCase {
    fn from(name: KebabCase) -> Self {
        let name = name.to_string();
        let mut characters = name.chars();
        let mut first_letter = true;
        let mut kebab_name = String::new();
        // FIXME: How can we clean up this code? :|
        while let Some(character) = characters.next() {
            if first_letter {
                kebab_name.push(character.to_ascii_uppercase());
                first_letter = false;
            } else {
                if character == '-' {
                    first_letter = true;
                } else {
                    kebab_name.push(character);
                }
            }
        }
        Self(kebab_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_kebab_case() {
        let kebab_case = KebabCase::try_from("naming-convention").expect("Not in snake case.");
        let pascal_case = PascalCase::from(kebab_case);
        assert_eq!(pascal_case.to_string(), "NamingConvention");
    }
}
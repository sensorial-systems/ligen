//! PascalCase.

use super::{NamingConvention, KebabCase, SnakeCase};
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
            NamingConvention::SnakeCase(name) => name.into(),
            _ => todo!("Implement other naming conventions.")
        }
    }
}

fn from_case_with_separator(case: String, separator: char) -> String {
    let name = case;
    let mut characters = name.chars();
    let mut first_letter = true;
    let mut pascal_case = String::new();
    // FIXME: How can we clean up this code? :|
    while let Some(character) = characters.next() {
        if first_letter {
            pascal_case.push(character.to_ascii_uppercase());
            first_letter = false;
        } else {
            if character == separator {
                first_letter = true;
            } else {
                pascal_case.push(character);
            }
        }
    }
    pascal_case
}

impl From<KebabCase> for PascalCase {
    fn from(name: KebabCase) -> Self {
        Self(from_case_with_separator(name.to_string(), '-'))
    }
}

impl From<SnakeCase> for PascalCase {
    fn from(name: SnakeCase) -> Self {
        Self(from_case_with_separator(name.to_string(), '_'))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_snake_case() {
        let snake_case = SnakeCase::try_from("naming_convention").expect("Not in snake case.");
        let pascal_case = PascalCase::from(snake_case);
        assert_eq!(pascal_case.to_string(), "NamingConvention");
    }

    #[test]
    fn from_kebab_case() {
        let kebab_case = KebabCase::try_from("naming-convention").expect("Not in snake case.");
        let pascal_case = PascalCase::from(kebab_case);
        assert_eq!(pascal_case.to_string(), "NamingConvention");
    }
}
//! Naming convetions such as kebab-case, snake_case, PascalCase, camelCase.

use crate::{prelude::*, Identifier};

/// Enumerated naming conventions.
#[derive(Debug, Clone, Serialize, Deserialize, Display, PartialEq, Eq)]
pub enum NamingConvention {
    /// kebab-case.
    KebabCase,

    /// snake_case.
    SnakeCase,

    /// SCREAMING_SNAKE_CASE.
    ScreamingSnakeCase,

    /// PascalCase.
    PascalCase,

    /// camelCase.
    CamelCase,

    /// Unknown naming convention.
    Unknown,
}

impl Identifier {
    /// Get the name convention of the Identifier.
    pub fn naming_convention(&self) -> NamingConvention {
        if self.name.contains('-') {
            NamingConvention::KebabCase
        } else if self.name.contains('_') {
            if self.name.chars().all(|c| !c.is_alphabetic() || c.is_uppercase()) {
                NamingConvention::ScreamingSnakeCase
            } else {
                NamingConvention::SnakeCase
            }
        } else if self.name.chars().next().unwrap().is_uppercase() {
            NamingConvention::PascalCase
        } else if self.name.chars().any(|c| c.is_uppercase()) {
            NamingConvention::CamelCase
        } else {
            NamingConvention::Unknown
        }
    }

    /// Set the name convention of the Identifier to camelCase.
    pub fn to_camel_case(&self) -> Self {
        let mut result = String::new();
        let mut first = true;
        for word in self.words() {
            if first {
                result.push_str(word.to_lowercase().as_str());
                first = false;
            } else {
                result.push_str(&word[..1].to_uppercase());
                result.push_str(&word[1..].to_lowercase());
            }
        }
        result.into()
    }

    /// Set the name convention of the Identifier to PascalCase.
    pub fn to_pascal_case(&self) -> Self {
        let mut result = String::new();
        for word in self.words() {
            result.push_str(&word[..1].to_uppercase());
            result.push_str(&word[1..].to_lowercase());
        }
        result.into()
    }

    /// Set the name convention of the Identifier to SCREAMING_SNAKE_CASE.
    pub fn to_screaming_snake_case(&self) -> Self {
        self.to_snake_case().name.to_uppercase().into()
    }

    /// Set the name convention of the Identifier to snake_case.
    pub fn to_snake_case(&self) -> Self {
        let mut result = String::new();
        let mut first = true;
        for word in self.words() {
            if first {
                result.push_str(word.to_lowercase().as_str());
                first = false;
            } else {
                result.push('_');
                result.push_str(word.to_lowercase().as_str());
            }
        }
        result.into()
    }

    /// Set the name convention of the Identifier to kebab-case.
    pub fn to_kebab_case(&self) -> Self {
        let mut result = String::new();
        let mut first = true;
        for word in self.words() {
            if first {
                result.push_str(word.to_lowercase().as_str());
                first = false;
            } else {
                result.push('-');
                result.push_str(word.to_lowercase().as_str());
            }
        }
        result.into()
    }

    /// Get the words of the Identifier.
    pub fn words(&self) -> Vec<&str> {
        match self.naming_convention() {
            NamingConvention::SnakeCase => self.name.split('_').collect(),
            NamingConvention::ScreamingSnakeCase => self.name.split('_').collect(),
            NamingConvention::KebabCase => self.name.split('-').collect(),
            NamingConvention::PascalCase => {
                let indices = self
                    .name
                    .chars()
                    .enumerate()
                    .filter(|(_, c)| c.is_uppercase())
                    .map(|(index, _)| index)
                    .chain(std::iter::once(self.name.len()))
                    .collect::<Vec<_>>();
                (0 .. indices.len() - 1)
                    .map(|i| {
                        &self.name[indices[i]..=(indices[i + 1] - 1)]
                    })
                    .collect()
            },
            NamingConvention::CamelCase => {
                let indices = 
                    std::iter::once(0)
                    .chain(self
                        .name
                        .chars()
                        .enumerate()
                        .filter(|(_, c)| c.is_uppercase())
                        .map(|(index, _)| index)
                        .chain(std::iter::once(self.name.len()))
                    ).collect::<Vec<_>>();
                (0 .. indices.len() - 1)
                    .map(|i| {
                        &self.name[indices[i]..=(indices[i + 1] - 1)]
                    })
                    .collect()
            },
            NamingConvention::Unknown => vec![self.name.as_str()],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Identifier;

    use super::*;

    #[test]
    fn screaming_snake_case() {
        assert_eq!(Identifier::from("SCREAMING_SNAKE_CASE").naming_convention(), NamingConvention::ScreamingSnakeCase);
    }

    #[test]
    fn kebab_case() {
        assert_eq!(Identifier::from("kebab-case").naming_convention(), NamingConvention::KebabCase);
    }

    #[test]
    fn snake_case() {
        assert_eq!(Identifier::from("snake_case").naming_convention(), NamingConvention::SnakeCase);
    }

    #[test]
    fn pascal_case() {
        assert_eq!(Identifier::from("PascalCase").naming_convention(), NamingConvention::PascalCase);
    }

    #[test]
    fn camel_case() {
        assert_eq!(Identifier::from("camelCase").naming_convention(), NamingConvention::CamelCase);
    }

    #[test]
    fn words() {
        assert_eq!(Identifier::from("case").words(), vec!["case"]);
        assert_eq!(Identifier::from("kebab-case").words(), vec!["kebab", "case"]);
        assert_eq!(Identifier::from("snake_case").words(), vec!["snake", "case"]);
        assert_eq!(Identifier::from("PascalCase").words(), vec!["Pascal", "Case"]);
        assert_eq!(Identifier::from("camelCase").words(), vec!["camel", "Case"]);
        assert_eq!(Identifier::from("SCREAMING_SNAKE_CASE").words(), vec!["SCREAMING", "SNAKE", "CASE"]);
    }

    #[test]
    fn convertion() {
        assert_eq!(Identifier::from("kebab-case").to_snake_case(), Identifier::from("kebab_case"));
        assert_eq!(Identifier::from("kebab-case").to_pascal_case(), Identifier::from("KebabCase"));
        assert_eq!(Identifier::from("kebab-case").to_camel_case(), Identifier::from("kebabCase"));
        assert_eq!(Identifier::from("kebab-case").to_screaming_snake_case(), Identifier::from("KEBAB_CASE"));

        assert_eq!(Identifier::from("snake_case").to_kebab_case(), Identifier::from("snake-case"));
        assert_eq!(Identifier::from("snake_case").to_pascal_case(), Identifier::from("SnakeCase"));
        assert_eq!(Identifier::from("snake_case").to_camel_case(), Identifier::from("snakeCase"));
        assert_eq!(Identifier::from("snake_case").to_screaming_snake_case(), Identifier::from("SNAKE_CASE"));

        assert_eq!(Identifier::from("PascalCase").to_kebab_case(), Identifier::from("pascal-case"));
        assert_eq!(Identifier::from("PascalCase").to_snake_case(), Identifier::from("pascal_case"));
        assert_eq!(Identifier::from("PascalCase").to_camel_case(), Identifier::from("pascalCase"));
        assert_eq!(Identifier::from("PascalCase").to_screaming_snake_case(), Identifier::from("PASCAL_CASE"));

        assert_eq!(Identifier::from("camelCase").to_kebab_case(), Identifier::from("camel-case"));
        assert_eq!(Identifier::from("camelCase").to_snake_case(), Identifier::from("camel_case"));
        assert_eq!(Identifier::from("camelCase").to_pascal_case(), Identifier::from("CamelCase"));
        assert_eq!(Identifier::from("camelCase").to_screaming_snake_case(), Identifier::from("CAMEL_CASE"));

        assert_eq!(Identifier::from("SCREAMING_SNAKE_CASE").to_kebab_case(), Identifier::from("screaming-snake-case"));
        assert_eq!(Identifier::from("SCREAMING_SNAKE_CASE").to_snake_case(), Identifier::from("screaming_snake_case"));
        assert_eq!(Identifier::from("SCREAMING_SNAKE_CASE").to_pascal_case(), Identifier::from("ScreamingSnakeCase"));
        assert_eq!(Identifier::from("SCREAMING_SNAKE_CASE").to_camel_case(), Identifier::from("screamingSnakeCase"));
    }
}
//! Naming convetions such as kebab-case, snake_case, PascalCase, camelCase.

mod kebab_case;
mod snake_case;
mod pascal_case;
mod camel_case;

pub use kebab_case::*;
pub use snake_case::*;
pub use pascal_case::*;
pub use camel_case::*;

use crate::prelude::*;

// TODO: Rework NamingConvention as `Name` and with methods identifying the convention.
//  Thinkg more about using `Name` with `Identifier`.

/// Enumerated naming conventions.
#[derive(Debug, Clone, Serialize, Deserialize, Display, PartialEq, Eq)]
pub enum NamingConvention {
    /// kebab-case.
    #[display(fmt = "{}", _0)]
    KebabCase(KebabCase),

    /// snake_case.
    #[display(fmt = "{}", _0)]
    SnakeCase(SnakeCase),

    /// PascalCase.
    #[display(fmt = "{}", _0)]
    PascalCase(PascalCase),

    /// camelCase.
    #[display(fmt = "{}", _0)]
    CamelCase(CamelCase)
}

impl Default for NamingConvention {
    fn default() -> Self {
        Self::KebabCase(Default::default())
    }
}

impl TryFrom<&str> for NamingConvention {
    type Error = Error;
    fn try_from(naming: &str) -> Result<Self> {
        if let Ok(kebab_case) = KebabCase::try_from(naming) {
            Ok(Self::KebabCase(kebab_case))
        } else if let Ok(camel_case) = CamelCase::try_from(naming) {
            Ok(Self::CamelCase(camel_case))
        } else if let Ok(pascal_case) = PascalCase::try_from(naming) {
            Ok(Self::PascalCase(pascal_case))
        } else if let Ok(snake_case) = SnakeCase::try_from(naming) {
            Ok(Self::SnakeCase(snake_case))
        } else {
            Err(Error::from(format!("Unknown naming convention: {}", naming)))
        }
    }
}

impl TryFrom<String> for NamingConvention {
    type Error = Error;
    fn try_from(naming: String) -> Result<Self> {
        Ok(naming.as_str().try_into()?)
    }
}

impl From<KebabCase> for NamingConvention {
    fn from(name: KebabCase) -> Self {
        Self::KebabCase(name)
    }
}

impl From<SnakeCase> for NamingConvention {
    fn from(name: SnakeCase) -> Self {
        Self::SnakeCase(name)
    }
}

impl From<CamelCase> for NamingConvention {
    fn from(name: CamelCase) -> Self {
        Self::CamelCase(name)
    }
}

impl From<PascalCase> for NamingConvention {
    fn from(name: PascalCase) -> Self {
        Self::PascalCase(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_string() {
        let name = "kebab-case";
        let naming_convention = NamingConvention::try_from(name).expect("Not in a known convention.");
        let kebab_case = KebabCase::try_from(name).expect("Not in kebab-case.");
        assert_eq!(naming_convention, kebab_case.into());
    }
}
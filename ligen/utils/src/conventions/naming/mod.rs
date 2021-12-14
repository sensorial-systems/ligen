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

impl TryFrom<&str> for NamingConvention {
    type Error = Error;
    fn try_from(naming: &str) -> Result<Self> {
        Ok(Self::KebabCase(naming.try_into()?))
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
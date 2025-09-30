use crate::prelude::*;
use crate::Type;

#[cfg(any(test, feature = "mocks"))]
pub mod mock;

/// Literal Enum
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, EnumAsInner, JsonSchema)]
#[serde(untagged)]
pub enum Literal {
    /// String variant
    String(String),
    /// Boolean variant
    Boolean(bool),
    /// Character variant
    Character(char),
    /// Integer variant
    Integer(i64),
    /// UnsignedInteger variant
    UnsignedInteger(u64),
    /// Float variant
    Float(f64),
    /// Tuple variant
    Tuple(Vec<Literal>),
    /// Array variant
    Array(Vec<Literal>),
    /// None variant
    None,
    /// Unknown variant used for language specific literals
    Unknown(String)
}

impl Literal {
    /// Check if `Literal` is compatible with `Type`.
    pub fn is_compatible_with(&self, type_: &Type) -> bool {
        match self {
            Literal::String(_) => type_.is_string() | !type_.is_primitive(),
            Literal::Boolean(_) => type_.is_boolean(),
            Literal::Character(_) => type_.is_character(),
            Literal::Integer(_) => type_.is_integer(),
            Literal::UnsignedInteger(_) => type_.is_unsigned_integer(),
            Literal::Float(_) => type_.is_float(),
            Literal::Tuple(_) => type_.is_tuple(),
            Literal::Array(_) => type_.is_array(),
            Literal::None => false,
            Literal::Unknown(_) => false
        }
    }

    pub fn default_for_type(type_: &Type) -> Self {
        if type_.is_string() {
            Self::String(Default::default())
        } else if type_.is_boolean() {
            Self::Boolean(false)
        } else if type_.is_character() {
            Self::Character('A')
        } else if type_.is_integer() {
            Self::Integer(0)
        } else if type_.is_unsigned_integer() {
            Self::UnsignedInteger(0)
        } else if type_.is_float() {
            Self::Float(0.0)
        } else {
            Self::None
        }
    }
}

impl Default for Literal {
    fn default() -> Self {
        Self::None
    }
}

impl From<&str> for Literal {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<&String> for Literal {
    fn from(value: &String) -> Self {
        value.to_string().into()
    }
}

impl From<String> for Literal {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<bool> for Literal {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<i32> for Literal {
    fn from(value: i32) -> Self {
        Self::Integer(value as i64)
    }
}

impl From<i64> for Literal {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<f32> for Literal {
    fn from(value: f32) -> Self {
        Self::Float(value as f64)
    }
}

impl From<f64> for Literal {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<u32> for Literal {
    fn from(value: u32) -> Self {
        Self::UnsignedInteger(value as u64)
    }
}

impl From<u64> for Literal {
    fn from(value: u64) -> Self {
        Self::UnsignedInteger(value)
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Literal::String(value) => write!(f, "{value}"),
            Literal::Boolean(value) => write!(f, "{value}"),
            Literal::Character(value) => write!(f, "{value}"),
            Literal::Integer(value) => write!(f, "{value}"),
            Literal::UnsignedInteger(value) => write!(f, "{value}"),
            Literal::Float(value) => write!(f, "{value}"),
            Literal::Tuple(values) => {
                write!(f, "(")?;
                for (index, value) in values.iter().enumerate() {
                    if index > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{value}")?;
                }
                write!(f, ")")
            },
            Literal::Array(values) => {
                write!(f, "[")?;
                for (index, value) in values.iter().enumerate() {
                    if index > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{value}")?;
                }
                write!(f, "]")
            },
            Literal::None => write!(f, "None"),
            Literal::Unknown(s) => write!(f, "Unknown({s})")
        }
    }
}

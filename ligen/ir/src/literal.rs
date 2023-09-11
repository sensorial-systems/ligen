use crate::prelude::*;

/// Literal Enum
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Literal {
    /// String variant
    String(String),
    /// Bool variant
    Bool(bool),
    /// Char variant
    Char(char),
    /// Integer variant
    Integer(i64),
    /// UnsignedInteger variant
    UnsignedInteger(u64),
    /// Float variant
    Float(f64),
}

impl Default for Literal {
    fn default() -> Self {
        Self::String(String::default())
    }
}

impl From<&str> for Literal {
    fn from(value: &str) -> Self {
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
        Self::Bool(value)
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
            Literal::String(value) => write!(f, "{}", value),
            Literal::Bool(value) => write!(f, "{}", value),
            Literal::Char(value) => write!(f, "{}", value),
            Literal::Integer(value) => write!(f, "{}", value),
            Literal::UnsignedInteger(value) => write!(f, "{}", value),
            Literal::Float(value) => write!(f, "{}", value),
        }
    }
}

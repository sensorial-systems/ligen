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

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

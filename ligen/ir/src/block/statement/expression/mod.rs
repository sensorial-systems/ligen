use crate::prelude::*;

pub mod binary;
pub use binary::*;

use ligen_idl::{Literal, Path, Identifier};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, JsonSchema)]
pub enum Expression {
    /// Path expression.
    Path(Path),
    /// Literal expression.
    Literal(Literal),
    /// Binary expression.
    Binary(BinaryExpression),
    /// Parenthesized expression.
    Parenthesized(Box<Expression>),
}

impl From<Identifier> for Expression {
    fn from(value: Identifier) -> Self {
        Self::Path(value.into())
    }
}

impl From<Path> for Expression {
    fn from(path: Path) -> Self {
        Self::Path(path)
    }
}

impl From<Literal> for Expression {
    fn from(literal: Literal) -> Self {
        Self::Literal(literal)
    }
}

impl From<BinaryExpression> for Expression {
    fn from(binary_expression: BinaryExpression) -> Self {
        Self::Binary(binary_expression)
    }
}

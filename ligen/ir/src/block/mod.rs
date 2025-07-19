use crate::prelude::*;

pub mod statement;
pub use statement::*;

/// Block structure.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Block {
    /// Statements field.
    pub statements: Vec<Statement>,
}

impl Block {
    pub fn new(statements: impl IntoIterator<Item = Statement>) -> Self {
        let statements = statements.into_iter().collect();
        Self { statements }
    }
}

impl<T: IntoIterator<Item = Statement>> From<T> for Block {
    fn from(statements: T) -> Self {
        Self::new(statements)
    }
}

impl From<Statement> for Block {
    fn from(statement: Statement) -> Self {
        Self::new(vec![statement])
    }
}
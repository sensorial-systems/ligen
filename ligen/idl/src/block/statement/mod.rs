use crate::prelude::*;

pub mod return_;
pub mod expression;

pub use return_::*;
pub use expression::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, JsonSchema)]
pub enum Statement {
    /// "return <expr>;" statement.
    Return(Return)
}

impl Statement {
    pub fn return_<T: Into<Expression>>(expression: Option<T>) -> Self {
        Self::Return(Return {
            value: expression.map(Into::into),
        })
    }
}
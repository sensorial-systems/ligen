use crate::prelude::*;

use crate::{Identifier, Expression};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BinaryExpression {
    /// Left operand field.
    pub left: Box<Expression>,
    /// Operator field.
    pub operator: Identifier,
    /// Right operand field.
    pub right: Box<Expression>,
}

impl BinaryExpression {
    pub fn new(left: impl Into<Expression>, operator: impl Into<Identifier>, right: impl Into<Expression>) -> Self {
        let left = Box::new(left.into());
        let right = Box::new(right.into());
        let operator = operator.into();
        Self { left, operator, right }
    }
}
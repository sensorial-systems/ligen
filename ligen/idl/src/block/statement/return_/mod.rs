use crate::prelude::*;

use crate::Expression;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Return {
    /// Return value field.
    pub value: Option<Expression>,
}
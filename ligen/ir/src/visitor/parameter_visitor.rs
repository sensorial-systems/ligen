use super::{Visitor, FunctionVisitor};
use crate::Parameter;

/// Parameter visitor.
pub type ParameterVisitor = Visitor<FunctionVisitor, Parameter>;

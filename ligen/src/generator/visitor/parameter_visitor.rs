use crate::generator::{Visitor, FunctionVisitor};
use crate::ir::Parameter;

/// Parameter visitor.
pub type ParameterVisitor = Visitor<FunctionVisitor, Parameter>;

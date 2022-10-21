use super::{Visitor, FunctionVisitor};
use ligen_ir::Parameter;

/// Parameter visitor.
pub type ParameterVisitor = Visitor<FunctionVisitor, Parameter>;

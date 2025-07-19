use super::{Visitor, FunctionVisitor};
use ligen_idl::Parameter;

/// Parameter visitor.
pub type ParameterVisitor = Visitor<FunctionVisitor, Parameter>;

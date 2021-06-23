//! Generator visitor module.

use crate::ir::{Implementation, Function, Parameter};

/// Generic visitor type.
#[derive(Debug, Clone)]
pub struct Visitor<Parent, Current> {
    parent: Parent,
    current: Current
}

/// Implementation visitor.
pub type ImplementationVisitor = Visitor<(), Implementation>;

/// Function visitor.
pub type FunctionVisitor = Visitor<Implementation, Function>;

/// Parameter visitor.
pub type ParameterVisitor = Visitor<Function, Parameter>;
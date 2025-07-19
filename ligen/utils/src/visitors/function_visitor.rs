use super::{Visitor, ModuleVisitor};
use ligen_idl::Function;

/// Function visitor.
pub type FunctionVisitor = Visitor<ModuleVisitor, Function>;

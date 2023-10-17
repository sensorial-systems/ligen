use super::{Visitor, ModuleVisitor};
use ligen_ir::Function;

/// Function visitor.
pub type FunctionVisitor = Visitor<ModuleVisitor, Function>;

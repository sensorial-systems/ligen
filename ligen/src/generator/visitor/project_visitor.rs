use crate::generator::Visitor;
use crate::ir::Project;

/// Project visitor.
pub type ProjectVisitor = Visitor<(), Project>;

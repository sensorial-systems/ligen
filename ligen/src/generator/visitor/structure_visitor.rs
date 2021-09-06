use crate::generator::{Visitor, ObjectVisitor};
use crate::ir::Structure;

/// Structure visitor.
pub type StructureVisitor = Visitor<ObjectVisitor, Structure>;

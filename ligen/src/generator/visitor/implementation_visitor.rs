use crate::generator::{Visitor, ObjectVisitor};
use crate::ir::Implementation;

/// Implementation visitor.
pub type ImplementationVisitor = Visitor<ObjectVisitor, Implementation>;

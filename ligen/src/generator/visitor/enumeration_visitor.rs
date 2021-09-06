use crate::generator::{Visitor, ObjectVisitor};
use crate::ir::Enumeration;

/// Enumeration visitor.
pub type EnumerationVisitor = Visitor<ObjectVisitor, Enumeration>;

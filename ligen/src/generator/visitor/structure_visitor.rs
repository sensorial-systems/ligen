use crate::generator::{Visitor, ObjectVisitor};
use crate::ir::{Structure, Path};

/// Structure visitor.
pub type StructureVisitor = Visitor<ObjectVisitor, Structure>;

impl StructureVisitor {
    /// Returns the module path.
    pub fn path(&self) -> Path {
        self.parent.path()
    }
}
use crate::generator::{Visitor, ObjectVisitor};
use crate::ir::{Implementation, Path};

/// Implementation visitor.
pub type ImplementationVisitor = Visitor<ObjectVisitor, Implementation>;

impl ImplementationVisitor {
    /// Returns the module path.
    pub fn path(&self) -> Path {
        self.parent.path()
    }
}
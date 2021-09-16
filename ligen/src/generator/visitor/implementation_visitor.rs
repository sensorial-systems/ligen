use crate::generator::{Visitor, ObjectVisitor, ModuleVisitor};
use crate::ir::{Implementation, Path};

/// Implementation visitor.
pub type ImplementationVisitor = Visitor<ObjectVisitor, Implementation>;

impl ImplementationVisitor {
    /// Returns the module path.
    pub fn path(&self) -> Path {
        self.parent.path()
    }

    /// Get the owner module.
    pub fn module(&self) -> &ModuleVisitor {
        &self.parent.parent
    }
}
use super::{Visitor, ModuleVisitor};
use ligen_ir::{Object, Path};

/// Object visitor.
pub type ObjectVisitor = Visitor<ModuleVisitor, Object>;

impl ObjectVisitor {
    /// Returns the object path.
    pub fn path(&self) -> Path {
        self.parent.path().join(self.identifier.clone())
    }

    /// Get the parent module.
    pub fn parent_module(&self) -> &ModuleVisitor {
        &self.parent
    }
}
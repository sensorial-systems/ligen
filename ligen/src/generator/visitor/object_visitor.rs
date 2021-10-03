use crate::generator::{Visitor, ModuleVisitor};
use crate::ir::{Object, Path};

/// Object visitor.
pub type ObjectVisitor = Visitor<ModuleVisitor, Object>;

impl ObjectVisitor {
    /// Returns the module path.
    pub fn path(&self) -> Path {
        self.parent.path().join(self.current.definition.identifier().clone())
    }

    /// Get the parent module.
    pub fn parent_module(&self) -> &ModuleVisitor {
        &self.parent
    }
}
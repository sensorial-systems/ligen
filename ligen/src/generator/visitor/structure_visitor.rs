use crate::generator::{Visitor, ObjectVisitor, ModuleVisitor};
use crate::ir::{Structure, Path};

/// Structure visitor.
pub type StructureVisitor = Visitor<ObjectVisitor, Structure>;

impl StructureVisitor {
    /// Returns the module path.
    pub fn path(&self) -> Path {
        self.parent.path()
    }

    /// Get the parent module.
    pub fn parent_module(&self) -> &ModuleVisitor {
        &self.parent.parent
    }

}
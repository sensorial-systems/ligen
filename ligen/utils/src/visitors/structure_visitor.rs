use super::{Visitor, ModuleVisitor};
use ligen_ir::{Structure, Path};

/// Structure visitor.
pub type StructureVisitor = Visitor<ModuleVisitor, Structure>;

impl StructureVisitor {
    /// Returns the module path.
    pub fn path(&self) -> Path {
        self.parent.path()
    }

    /// Get the parent module.
    pub fn parent_module(&self) -> &ModuleVisitor {
        &self.parent
    }
}
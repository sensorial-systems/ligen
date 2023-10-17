use super::{Visitor, ModuleVisitor};
use ligen_ir::{Enumeration, Path};

/// Enumeration visitor.
pub type EnumerationVisitor = Visitor<ModuleVisitor, Enumeration>;

impl EnumerationVisitor {
    /// Returns the enumeration path.
    pub fn path(&self) -> Path {
        self.parent_module().path().join(self.parent.identifier.clone())
    }

    /// Get the parent module.
    pub fn parent_module(&self) -> &ModuleVisitor {
        &self.parent
    }
}
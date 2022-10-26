use super::{Visitor, ObjectVisitor, ModuleVisitor};
use ligen_ir::{Enumeration, Path};

/// Enumeration visitor.
pub type EnumerationVisitor = Visitor<ObjectVisitor, Enumeration>;

impl EnumerationVisitor {
    /// Returns the enumeration path.
    pub fn path(&self) -> Path {
        self.current.path.clone()
    }

    /// Get the parent module.
    pub fn parent_module(&self) -> &ModuleVisitor {
        self.parent.parent_module()
    }
}
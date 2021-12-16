use super::{Visitor, ObjectVisitor, ModuleVisitor};
use crate::{Enumeration, Path};

/// Enumeration visitor.
pub type EnumerationVisitor = Visitor<ObjectVisitor, Enumeration>;

impl EnumerationVisitor {
    /// Returns the module path.
    pub fn path(&self) -> Path {
        self.parent.path().join(self.current.identifier.clone())
    }

    /// Get the parent module.
    pub fn parent_module(&self) -> &ModuleVisitor {
        self.parent.parent_module()
    }
}
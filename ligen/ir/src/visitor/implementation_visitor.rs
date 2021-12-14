use super::{Visitor, ObjectVisitor, ModuleVisitor};
use crate::{Implementation, Path};

/// Implementation visitor.
pub type ImplementationVisitor = Visitor<ObjectVisitor, Implementation>;

impl ImplementationVisitor {
    /// Returns the module path.
    pub fn path(&self) -> Path {
        self.parent.path()
    }

    /// Get the parent module.
    pub fn parent_module(&self) -> &ModuleVisitor {
        &self.parent.parent
    }
}
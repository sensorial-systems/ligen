use crate::generator::{Visitor, ModuleVisitor};
use crate::ir::{Object, Path};

/// Object visitor.
pub type ObjectVisitor = Visitor<ModuleVisitor, Object>;

impl ObjectVisitor {
    /// Returns the module path.
    pub fn path(&self) -> Path {
        let mut segments = self.parent.path().segments;
        segments.append(&mut self.current.path.segments.clone());
        segments.into()
    }
}
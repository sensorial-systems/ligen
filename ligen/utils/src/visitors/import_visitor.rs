use ligen_ir::{Import, Path, Library};
use crate::visitors::{ModuleVisitor, Visitor};

pub type ImportVisitor = Visitor<ModuleVisitor, Import>;

impl ImportVisitor {
    /// Returns the parent library.
    pub fn parent_library(&self) -> &Library {
        self.parent.parent_library()
    }

    pub fn find_absolute_path(&self) -> Option<Path> {
        self.parent.find_absolute_path(&self.current.path)
    }
}
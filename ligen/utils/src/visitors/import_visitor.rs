use ligen_ir::{Import, Path, Project};
use crate::visitors::{ModuleVisitor, Visitor};

pub type ImportVisitor = Visitor<ModuleVisitor, Import>;

impl ImportVisitor {
    /// Returns the parent project.
    pub fn parent_project(&self) -> &Project {
        self.parent.parent_project()
    }

    pub fn find_absolute_path(&self) -> Option<Path> {
        self.parent.find_absolute_path(&self.current.path)
    }
}
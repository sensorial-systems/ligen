use super::Visitor;
use ligen_ir::Project;
use crate::visitors::ModuleVisitor;

/// Project visitor.
pub type ProjectVisitor = Visitor<(), Project>;

impl From<Project> for ProjectVisitor {
    fn from(project: Project) -> Self {
        Self::new((), project)
    }
}

impl ProjectVisitor {
    pub fn root_module_visitor(&self) -> ModuleVisitor {
        (&self.child(self.current.root_module.clone())).into()
    }
}

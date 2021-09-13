use crate::generator::Visitor;
use crate::ir::Project;

/// Project visitor.
pub type ProjectVisitor = Visitor<(), Project>;

impl From<Project> for ProjectVisitor {
    fn from(project: Project) -> Self {
        Self::new((), project)
    }
}
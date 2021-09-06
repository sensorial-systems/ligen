use crate::generator::{ProjectVisitor, Visitor};
use crate::ir::{Module, Project, Path};
use crate::conventions::naming::SnakeCase;

/// All the possibilities of module parents.
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub enum ModuleParent {
    Project(ProjectVisitor),
    Module(Box<ModuleVisitor>)
}

/// Module visitor.
pub type ModuleVisitor = Visitor<ModuleParent, Module>;

impl ModuleVisitor {
    /// Returns the parent project.
    pub fn parent_project(&self) -> &Project {
        match &self.parent {
            ModuleParent::Project(visitor) => &visitor.current,
            ModuleParent::Module(module) => module.parent_project()
        }
    }

    /// Returns the module path.
    pub fn path(&self) -> Path {
        let mut segments = Vec::new();
        match &self.parent {
            ModuleParent::Project(project) => segments.push(SnakeCase::from(project.current.name().clone()).into()),
            ModuleParent::Module(module) => {
                segments.append(&mut module.path().segments);
                segments.push(self.current.name.clone());
            }
        }
        segments.into()
    }
}

impl From<&Visitor<ProjectVisitor, Module>> for ModuleVisitor {
    fn from(visitor: &Visitor<ProjectVisitor, Module>) -> Self {
        let parent = ModuleParent::Project(visitor.parent.clone());
        let current = visitor.current.clone();
        Self { parent, current }
    }
}

impl From<&Visitor<ModuleVisitor, Module>> for ModuleVisitor {
    fn from(visitor: &Visitor<ModuleVisitor, Module>) -> Self {
        let parent = ModuleParent::Module(visitor.parent.clone().into());
        let current = visitor.current.clone();
        Self { parent, current }
    }
}

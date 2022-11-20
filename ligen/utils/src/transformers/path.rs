// use ligen_ir::Project;
// use crate::visitors::{ModuleVisitor, ProjectVisitor, Visitor};

use ligen_ir::{Import, Module, Project};
use crate::transformers::Transform;
use crate::visitors::{ImportVisitor, ModuleVisitor, ProjectVisitor};

pub struct RelativePathToAbsolutePath;

impl Transform<Project, Project> for RelativePathToAbsolutePath {
    fn transform(&self, data: &Project) -> Project {
        let mut data = data.clone();
        data.root_module.guarantee_absolute_paths();
        let visitor = ProjectVisitor::from(data);
        <Self as Transform<ProjectVisitor, Project>>::transform(self, &visitor)
    }
}

impl Transform<ProjectVisitor, Project> for RelativePathToAbsolutePath {
    fn transform(&self, data: &ProjectVisitor) -> Project {
        let mut project = data.current.clone();
        let visitor = ModuleVisitor::from(&data.child(data.current.root_module.clone()));
        project.root_module = <Self as Transform::<ModuleVisitor, Module>>::transform(self, &visitor);
        project
    }
}

impl Transform<ModuleVisitor, Module> for RelativePathToAbsolutePath {
    fn transform(&self, data: &ModuleVisitor) -> Module {
        let mut module = data.current.clone();
        for (index, import) in data.imports.iter().enumerate() {
            let visitor = data.child(import.clone());
            module.imports[index] = <Self as Transform::<ImportVisitor, Import>>::transform(self, &visitor);
        }
        for (index, child_module) in data.modules.iter().enumerate() {
            let visitor = ModuleVisitor::from(&data.child(child_module.clone()));
            module.modules[index] = <Self as Transform::<ModuleVisitor, Module>>::transform(self, &visitor);
        }
        module
    }
}

impl Transform<ImportVisitor, Import> for RelativePathToAbsolutePath {
    fn transform(&self, data: &ImportVisitor) -> Import {
        let mut import = data.current.clone();
        if let Some(absolute_path) = data.find_absolute_path() {
            import.path = absolute_path;
        }
        import
    }
}

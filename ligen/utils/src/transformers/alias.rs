use ligen_ir::{Import, Module, Project};
use crate::transformers::Transform;
use crate::visitors::{ImportVisitor, ModuleVisitor, ProjectVisitor};

// FIXME: Move this to ligen-rust.
pub struct ReplaceCrateAlias;

// FIXME: This logic is duplicated fro other transformers. This could be somehow generalized.

impl Transform<Project, Project> for ReplaceCrateAlias {
    fn transform(&self, data: &Project) -> Project {
        let visitor = ProjectVisitor::from(data.clone());
        <Self as Transform::<ProjectVisitor, Project>>::transform(self, &visitor)
    }
}

impl Transform<ProjectVisitor, Project> for ReplaceCrateAlias {
    fn transform(&self, data: &ProjectVisitor) -> Project {
        let mut project = data.current.clone();
        let visitor = ModuleVisitor::from(&data.child(data.current.root_module.clone()));
        project.root_module = <Self as Transform::<ModuleVisitor, Module>>::transform(self, &visitor);
        project
    }
}

impl Transform<ModuleVisitor, Module> for ReplaceCrateAlias {
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

impl Transform<ImportVisitor, Import> for ReplaceCrateAlias {
    fn transform(&self, data: &ImportVisitor) -> Import {
        let mut import = data.current.clone();
        let first = import.path.first_mut();
        if *first == "crate".into() {
            *first = data.parent_project().root_module.identifier.clone().into();
        }
        import
    }
}

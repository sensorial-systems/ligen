use ligen_ir::{Function, Import, Module, Project, Type};
use crate::transformers::Transform;
use crate::visitors::{FunctionVisitor, ImportVisitor, ModuleVisitor, ProjectVisitor};

pub struct RelativePathToAbsolutePath;

impl Transform<Project, Project> for RelativePathToAbsolutePath {
    fn transform(&self, data: &Project) -> Project {
        let data = data.clone();
        // TODO: We need to review this process.
        // data.root_module.guarantee_absolute_paths();
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
        for (index, import) in data.current.imports.iter().enumerate() {
            let visitor = data.child(import.clone());
            module.imports[index] = <Self as Transform::<ImportVisitor, Import>>::transform(self, &visitor);
        }
        for (index, function) in data.current.functions.iter().enumerate() {
            let visitor = data.child(function.clone());
            module.functions[index] = <Self as Transform::<FunctionVisitor, Function>>::transform(self, &visitor);
        }
        for (index, child_module) in data.current.modules.iter().enumerate() {
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

impl Transform<FunctionVisitor, Function> for RelativePathToAbsolutePath {
    fn transform(&self, data: &FunctionVisitor) -> Function {
        let mut function = data.current.clone();
        if let Some(output) = function.output.as_mut() {
            type_to_absolute_path(&data.parent, output);
        }
        for input in &mut function.inputs {
            type_to_absolute_path(&data.parent, &mut input.type_);
        }
        function
    }
}

fn type_to_absolute_path(module_visitor: &ModuleVisitor, type_: &mut Type) {
    match type_ {
        Type::Composite(path, _) => {
            if let Some(absolute_path) = module_visitor.find_absolute_path(path) {
                *path = absolute_path
            }
        },
        Type::Reference(reference) => {
            type_to_absolute_path(module_visitor, &mut reference.type_)
        },
        _ => ()
    }
}
use ligen_ir::{Function, Import, Module, Library, Type};
use crate::transformers::Transform;
use crate::visitors::{FunctionVisitor, ImportVisitor, ModuleVisitor, LibraryVisitor};

pub struct RelativePathToAbsolutePath;

impl Transform<Library, Library> for RelativePathToAbsolutePath {
    fn transform(&self, data: &Library) -> Library {
        let data = data.clone();
        // TODO: We need to review this process.
        // data.root_module.guarantee_absolute_paths();
        let visitor = LibraryVisitor::from(data);
        <Self as Transform<LibraryVisitor, Library>>::transform(self, &visitor)
    }
}

impl Transform<LibraryVisitor, Library> for RelativePathToAbsolutePath {
    fn transform(&self, data: &LibraryVisitor) -> Library {
        let mut library = data.current.clone();
        let visitor = ModuleVisitor::from(&data.child(data.current.root_module.clone()));
        library.root_module = <Self as Transform::<ModuleVisitor, Module>>::transform(self, &visitor);
        library
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
    let path = &mut type_.path;
    if let Some(absolute_path) = module_visitor.find_absolute_path(path) {
        *path = absolute_path
    }
}
use ligen_ir::{Import, Module, Library};
use crate::transformers::Transform;
use crate::visitors::{ImportVisitor, ModuleVisitor, LibraryVisitor};

// FIXME: Move this to ligen-rust.
pub struct ReplaceCrateAlias;

// FIXME: This logic is duplicated fro other transformers. This could be somehow generalized.

impl Transform<Library, Library> for ReplaceCrateAlias {
    fn transform(&self, data: &Library) -> Library {
        let visitor = LibraryVisitor::from(data.clone());
        <Self as Transform::<LibraryVisitor, Library>>::transform(self, &visitor)
    }
}

impl Transform<LibraryVisitor, Library> for ReplaceCrateAlias {
    fn transform(&self, data: &LibraryVisitor) -> Library {
        let mut library = data.current.clone();
        let visitor = ModuleVisitor::from(&data.child(data.current.root_module.clone()));
        library.root_module = <Self as Transform::<ModuleVisitor, Module>>::transform(self, &visitor);
        library
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
            *first = data.parent_library().root_module.identifier.clone().into();
        }
        import
    }
}

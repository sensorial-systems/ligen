use super::{LibraryVisitor, Visitor};
use ligen_ir::{Module, Library, Path};

/// All the possibilities of module parents.
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub enum ModuleParent {
    Library(LibraryVisitor),
    Module(Box<ModuleVisitor>)
}

/// Module visitor.
pub type ModuleVisitor = Visitor<ModuleParent, Module>;

impl ModuleVisitor {
    /// Get the module path.
    pub fn path(&self) -> Path {
        match &self.parent {
            ModuleParent::Library(visitor) => visitor.current.root_module.identifier.clone().into(),
            ModuleParent::Module(module) => module.path().clone().join(self.identifier.clone())
        }
    }

    /// Returns the parent library.
    pub fn parent_library(&self) -> &Library {
         match &self.parent {
            ModuleParent::Library(visitor) => &visitor.current,
            ModuleParent::Module(module) => module.parent_library()
        }
    }

    /// Get the parent module.
    pub fn parent_module(&self) -> Option<&ModuleVisitor> {
        match &self.parent {
            ModuleParent::Module(module) => Some(module),
            ModuleParent::Library(_) => None
        }
    }
}

impl ModuleVisitor {
    // TODO: We need to review this process.
    pub fn find_absolute_path(&self, relative_path: &Path) -> Option<Path> {
        let mut consumed_relative_path = relative_path.clone();
        // path is not empty
        if let Some(path_segment) = consumed_relative_path.pop_front() {
            let library = self.parent_library();
            let root_module_name = &library.root_module.identifier;
            // the first segment can be either: crate, self, super, a module or the definition itself.
            // self module
            if path_segment == "self".into() {
                self.find_absolute_path(&consumed_relative_path)
            // root module
            } else if path_segment.identifier == *root_module_name {
                let visitor = LibraryVisitor::from(library.clone()).root_module_visitor();
                visitor.find_absolute_path(&consumed_relative_path)
            // super module
            } else if path_segment == "super".into() {
                if let Some(visitor) = self.parent_module() {
                    visitor.find_absolute_path(&consumed_relative_path)
                } else {
                    None
                }
            // module
            } else if !consumed_relative_path.segments.is_empty() {
                let sub_module = self
                    .current
                    .modules
                    .iter()
                    .find(|module| module.identifier == path_segment.identifier)
                    .map(|module| ModuleVisitor::from(&self.child(module.clone())));
                // sub module
                if let Some(sub_module) = sub_module {
                    sub_module.find_absolute_path(&consumed_relative_path)
                // imported module
                } else {
                    self
                        .current
                        .imports
                        .iter()
                        .find(|import| *import.path.last() == path_segment)
                        .and_then(|import| self.find_absolute_path(&import.path.clone().join(consumed_relative_path.clone())))
                        // it's an external module and we have the item full path in the relative_path if we got here.
                        .or(Some(relative_path.clone()))
                }
            // import or type definition
            } else {
                self
                    .current
                    .imports
                    .iter()
                    .find(|import| (*import.path.last() == path_segment && import.renaming.is_none()) || import.renaming.as_ref() == Some(&path_segment.identifier))
                    // import
                    .and_then(|import| self.find_absolute_path(&import.path.clone()))
                    // type definition
                    // .or(Some(self.current.path.clone().join(identifier))) // FIXME: This is not correct.
                    .or(None)
            }
            // path is empty
        } else {
            // Some(self.current.path.clone()) // FIXME: This is not correct.
            None
        }
    }
}

impl From<&Visitor<LibraryVisitor, Module>> for ModuleVisitor {
    fn from(visitor: &Visitor<LibraryVisitor, Module>) -> Self {
        let parent = ModuleParent::Library(visitor.parent.clone());
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

#[cfg(test)]
mod tests {
    // #[test]
    // fn path_solver() {
    //     let root_module = quote! {
    //         pub mod lib {
    //             pub mod objects {
    //                 pub struct Object {
    //                 }
    //             }
    //             mod other_objects {
    //                 pub struct OtherObject {
    //                 }
    //                 pub enum OtherEnum {
    //                     A,
    //                     B
    //                 }
    //             }
    //             pub use objects::Object;
    //             pub use other_objects::*;
    //             pub mod usage {
    //                 pub use crate::Object;
    //                 pub use super::OtherEnum;
    //             }
    //             pub mod use_all {
    //                 pub use crate::*;
    //             }
    //         }
    //     });
    // }
}

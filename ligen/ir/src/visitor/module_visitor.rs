use super::{ProjectVisitor, Visitor};
use crate::{Module, Project, Path};
use ligen_utils::conventions::naming::SnakeCase;

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

    /// Get the parent module.
    pub fn parent_module(&self) -> Option<&ModuleVisitor> {
        match &self.parent {
            ModuleParent::Module(module) => Some(module),
            ModuleParent::Project(_) => None
        }
    }
}

impl ModuleVisitor {

    /// Find absolute path.
    pub fn find_absolute_path(&self, relative_path: &Path) -> Option<Path> {
        let mut relative_path = relative_path.clone();
        // path is not empty
        if let Some(identifier) = relative_path.pop_front() {
            println!("Looking for {} in {}", identifier, self.current.name);
            // the first segment can be either: root, self, super, a module or the definition itself.
            // self module
            if identifier == "self".into() {
                self.find_absolute_path(&relative_path)
            // root module
            } else if identifier == "root".into() {
                self.parent_project().root_module_visitor().find_absolute_path(&relative_path)
            // super module
            } else if identifier == "super".into() {
                self
                    .parent_module()
                    .and_then(|module| module.find_absolute_path(&relative_path))
            // sub module
            } else if !relative_path.segments.is_empty() {
                println!("Looking for {} in {:#?}", relative_path, self.current.modules.len());
                self
                    .current
                    .modules
                    .iter()
                    .filter(|module| {
                        println!("{} == {}", module.name, identifier);
                        module.name == identifier
                    })
                    .map(|module| ModuleVisitor::from(&self.child(module.clone())))
                    .filter_map(|module| {
                        println!("Entering relative_path: {}", relative_path);
                        module.find_absolute_path(&relative_path)
                    })
                    .next()
            // import or type definition
            } else {
                // look for definition
                let definition = self
                    .current
                    .objects
                    .iter()
                    .filter(|object| *object.definition.identifier() == identifier)
                    .next()
                    .map(|_| Path::from("root").join(self.path().join(identifier.clone()).without_first()));
                if definition.is_some() {
                    definition
                // look for imports
                } else {
                    self
                        .current
                        .imports
                        .iter()
                        .filter(|import| {
                            if let Some(renamed) = &import.renaming {
                                identifier == *renamed
                            } else {
                                import.path.last() == identifier
                            }
                        })
                        .map(|import| &import.path)
                        .next()
                        .and_then(|path| self.find_absolute_path(path))
                }
            }
        // path is empty
        } else {
            None
        }
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
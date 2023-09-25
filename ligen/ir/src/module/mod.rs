//! Module representation.

pub mod import;

#[cfg(any(test, feature = "mocks"))]
pub mod mock;

pub use import::*;

use crate::prelude::*;
use crate::{Object, Path, Visibility, Attributes, Function, Literal, Constant, Identifier};

/// Module representation.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Module {
    /// Attributes.
    pub attributes: Attributes,
    /// Visibility.
    pub visibility: Visibility,
    /// Module identifier
    pub identifier: Identifier,
    /// Imports.
    pub imports: Vec<Import>,
    /// Constants.
    pub constants: Vec<Constant>,
    /// Functions.
    pub functions: Vec<Function>,
    /// Objects.
    pub objects: Vec<Object>,
    /// Sub-modules.
    pub modules: Vec<Module>,
}

impl Module {
    pub fn resolve_paths(&mut self, root: &Self) {
        for import in &mut self.imports {
            println!("{}", import.path);
        }
        for module in &mut self.modules {
            module.resolve_paths(root);
        }
    }

    /// FIXME: This is a temporary workaround.
    pub fn get_attributes_from_path<P: Into<Path>>(&self, path: P) -> Option<&Attributes> {
        let path = path.into();
        if let Some(attributes) = self.attributes.get_subgroup(path.clone()) {
            Some(attributes)
        } else {
            self
                .modules
                .iter()
                .find_map(|module| module.get_attributes_from_path(path.clone()))
        }
    }

    /// FIXME: This is a temporary workaround.
    pub fn get_literal_from_path<P: Into<Path>>(&self, path: P) -> Option<&Literal> {
        let path = path.into();
        if let Some(literal) = self.attributes.get_literal_from_path(path.clone()) {
            Some(literal)
        } else {
            self
                .modules
                .iter()
                .find_map(|module| module.get_literal_from_path(path.clone()))
        }
    }

    /// Tells if ligen is ignoring this module.
    pub fn ignored(&self) -> bool {
        self.attributes.has_ignore_attribute()
    }

    /// Find mutable Object.
    pub fn find_object_mut(&mut self, path: &Path) -> Option<&mut Object> {
        let mut path = path.clone();
        if let Some(identifier) = path.pop_back() {
            let object = self
                .objects
                .iter_mut()
                .find(|object| object.identifier == identifier);
            if let Some(object) = object {
                Some(object)
            } else {
                self
                    .modules
                    .iter_mut()
                    .filter_map(|module| module.find_object_mut(&path))
                    .next()
            }
        } else {
            None
        }
    }

    /// Find Object.
    pub fn find_object(&self, path: &Path) -> Option<&Object> {
        let mut path = path.clone();
        if let Some(identifier) = path.pop_back() {
            let object = self
                .objects
                .iter()
                .find(|object| object.identifier == identifier);
            if let Some(object) = object {
                Some(object)
            } else {
                self
                    .modules
                    .iter()
                    .find(|module| module.identifier == identifier)
                    .and_then(|module| module.find_object(&path))
            }
        } else {
            None
        }
    }
}

impl Module {
    /// Find the module with the specified path.
    pub fn find_module(&self, path: &Path) -> Option<&Module> {
        let mut path = path.clone();
        if let Some(identifier) = path.pop_back() {
            if path.segments.is_empty() && self.identifier == identifier {
                Some(self)
            } else {
                self
                    .modules
                    .iter()
                    .filter_map(|module| module.find_module(&path))
                    .next()
            }
        } else {
            None
        }
    }

    // FIXME: Move this to `Project`'s post-processing.
    /// Replace wild card imports with actual imports.
    pub fn replace_wildcard_imports(&mut self) {
        for module in &mut self.modules {
            module.replace_wildcard_imports();
        }
        let wildcard_imports: Vec<_> = self
            .imports
            .iter()
            .filter(|import| import.path.last() == "*".into())
            .cloned()
            .collect();
        let mut imports: Vec<_> = self
            .imports
            .iter()
            .filter(|import| import.path.last() != "*".into())
            .cloned()
            .collect();
        for import in wildcard_imports {
            let module_path = import.path.clone().without_last();
            println!("ModulePath: {}", module_path);
            if let Some(module) = self.find_module(&module_path) {
                for object in &module.objects {
                    if let Visibility::Public = object.visibility {
                        imports.push(Import {
                            attributes: import.attributes.clone(),
                            visibility: import.visibility.clone(),
                            renaming: import.renaming.clone(),
                            path: object.identifier.clone().into() // FIXME: This is a temporary workaround. Identifier should be a Path.
                        })
                    }
                }
                for internal_import in &module.imports {
                    if let Visibility::Public = internal_import.visibility {
                        let identifier = if let Some(renaming) = &internal_import.renaming {
                            renaming.clone()
                        } else {
                            internal_import.path.last()
                        };
                        imports.push(Import {
                            attributes: import.attributes.clone(),
                            visibility: import.visibility.clone(),
                            renaming: import.renaming.clone(),
                            path: module_path.clone().join(identifier)
                        })
                    }
                }
            }
        }
        self.imports = imports;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    #[test]
    fn object_finder() -> Result<()> {
        let module = Module {
            identifier: "types".into(),
            visibility: Visibility::Public,
            objects: vec![
                Object {
                    visibility: Visibility::Public,
                    identifier: "Type".into(),
                    .. Default::default()
                }
            ],
            ..Default::default()
        };
        let object = module.find_object(&"Type".into());
        let expected_object = Some(Object {
            visibility: Visibility::Public,
            identifier: "Type".into(),
            definition: Structure::default().into(),
            .. Default::default()
        });
        assert_eq!(object, expected_object.as_ref());
        Ok(())
    }
}
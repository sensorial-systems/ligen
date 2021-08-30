//! Module representation.

use crate::prelude::*;
use crate::ir::{Object, Path, Structure, Implementation, Visibility, Identifier};
use std::convert::TryFrom;
use std::collections::HashMap;
use std::io::Read;
use std::fs::File;

/// Module representation.
#[derive(Debug, Clone, PartialEq)]
pub struct Module {
//    /// Module path.
//    pub path: Path,
    /// Visibility.
    pub visibility: Visibility,
    /// Module name.
    pub name: Identifier,
    /// Sub-modules.
    pub modules: Vec<Module>,
    /// Objects.
    pub objects: Vec<Object>
}

impl TryFrom<&std::path::Path> for Module {
    type Error = Error;
    fn try_from(from: &std::path::Path) -> Result<Self> {
        // FIXME: This function needs a cleanup.
        println!("Path: {}", from.display());
        let mut file = File::open(from)?;
        let mut src = String::new();
        file.read_to_string(&mut src)?;
        let file = syn::parse_file(&src)?;
        let visibility = Visibility::Public;
        let parent_path = from.parent().expect("Failed to get parent path.");

        // FIXME: This is repetitive.
        let mut name = from
            .file_stem()
            .and_then(|name| name.to_str())
            .map(|name| name.to_string())
            .filter(|name| name != "mod");
        if name.is_none() {
            name = parent_path
                .file_stem()
                .and_then(|parent| parent.to_str())
                .map(|parent| parent.to_string());
        }
        let name = name
            .ok_or_else(|| Error::Message("Couldn't get module name.".into()))?
            .into();

        // FIXME: This needs a better generalization.
        // println!("Parent: {}", parent_path.display());
        let base_path = if from.ends_with("src/lib.rs") || from.ends_with("mod.rs") {
            from.parent().expect("Couldn't get parent.").to_path_buf()
        } else {
            from.with_extension("")
        };
        let modules = Module::parse_modules(&file.items, base_path.as_path())?;
        let objects = Module::parse_objects(&file.items)?;
        Ok(Module { visibility, name, modules, objects })
    }
}

impl Module {
    /// Gets the root module (lib.rs).
    pub fn root() -> Result<Self> {
        std::path::Path::new("src").join("lib.rs").as_path().try_into()
    }
}

impl Module {
    /// Replace all the occurrences of `Self` by the real object name.
    /// e.g.:
    /// ```rust,compile_fail
    /// impl Object {
    ///     fn f(self: &Self) {}
    /// }
    /// ```
    /// becomes
    /// ```rust,compile_fail
    /// impl Object {
    ///     fn f(self: &Object) {}
    /// }
    /// ```
    pub fn replace_self_with_explicit_names(&mut self) {
        for module in &mut self.modules {
            module.replace_self_with_explicit_names();
        }
        for object in &mut self.objects {
            for implementation in &mut object.implementations {
                implementation.replace_self_with_explicit_names();
            }
        }
    }
}

impl Module {
    fn parse_modules(items: &Vec<syn::Item>, base_path: &std::path::Path) -> Result<Vec<Module>> {
        let mut modules = Vec::new();
        for item in items {
            match item {
                syn::Item::Mod(module) => modules.push(Module::try_from((module.clone(), base_path))?),
                _ => ()
            }
        }
        Ok(modules)
    }

    fn parse_objects(items: &Vec<syn::Item>) -> Result<Vec<Object>> {
        let mut objects: HashMap<Path, (Option<Structure>, Vec<Implementation>)> = HashMap::new();
        for item in items {
            match item {
                syn::Item::Struct(structure) => {
                    let structure = Structure::try_from(structure.clone())?;
                    let path = structure.identifier.clone().into();
                    if let Some((optional_structure, _implementations)) = objects.get_mut(&path) {
                        *optional_structure = Some(structure);
                    } else {
                        objects.insert(path, (Some(structure), Default::default()));
                    }
                },
                syn::Item::Impl(implementation) => {
                    // TODO: Consider `impl Trait for Object`?
                    if implementation.trait_.is_none() {
                        let implementation = Implementation::try_from(implementation.clone())?;
                        let path = implementation.self_.path();
                        if let Some((_structure, implementations)) = objects.get_mut(&path) {
                            implementations.push(implementation);
                        } else {
                            objects.insert(path, (None, vec![implementation]));
                        }
                    }
                }
                _ => ()
            }
        }
        let mut objects: Vec<_> = objects
            .into_iter()
            .map(|(path, (structure, implementations))| Object {
                path,
                structure,
                implementations
            })
            .collect();
        objects.sort_by(|a, b| a.path.cmp(&b.path));
        Ok(objects)
    }
}

impl TryFrom<(syn::ItemMod, &std::path::Path)> for Module {
    type Error = Error;
    fn try_from(module: (syn::ItemMod, &std::path::Path)) -> Result<Self> {
        let (module, base_path) = module;
        let base_path = base_path.join(module.ident.to_string());
        if let Some((_, items)) = module.content {
            let modules = Module::parse_modules(&items, base_path.as_path())?;
            let objects = Module::parse_objects(&items)?;
            let name = module.ident.into();
            let visibility = module.vis.into();
            Ok(Self { visibility, name, modules, objects })
        } else {
            let mut path = base_path.with_extension("rs");
            if !path.exists() {
                path = base_path.join("mod.rs");
            }
            let path = path.as_path();
            path.try_into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{Object, Atomic, Integer, Type, Visibility, Function, Structure, Parameter, Implementation, ImplementationItem, Field};
    use quote::quote;

    #[test]
    fn object() {
        assert_eq!(
            Module::try_from(quote! {
                pub struct Object {
                    pub integer: i32
                }

                impl Object {
                    pub fn new(integer: i32) -> Self {
                        Self { integer }
                    }
                }

                pub struct AnotherObject;
            }).expect("Failed to convert from ItemImpl"),
            Module {
                objects: vec![
                    Object {
                        path: "AnotherObject".into(),
                        structure: Some(Structure {
                            attributes: Default::default(),
                            visibility: Visibility::Public,
                            identifier: "AnotherObject".into(),
                            fields: Default::default(),
                        }),
                        implementations: Default::default()
                    },
                    Object {
                        path: "Object".into(),
                        structure: Some(Structure {
                            attributes: Default::default(),
                            visibility: Visibility::Public,
                            identifier: "Object".into(),
                            fields: vec![
                                Field {
                                    attributes: Default::default(),
                                    visibility: Visibility::Public,
                                    identifier: "integer".into(),
                                    type_: Type::Atomic(Atomic::Integer(Integer::I32))
                                }
                            ]
                        }),
                        implementations: vec![
                            Implementation {
                                attributes: Default::default(),
                                self_: Type::Compound("Object".into()),
                                items: vec![
                                    ImplementationItem::Method(Function {
                                        attributes: Default::default(),
                                        visibility: Visibility::Public,
                                        asyncness: None,
                                        identifier: "new".into(),
                                        inputs: vec![
                                            Parameter {
                                                identifier: "integer".into(),
                                                type_: Type::Atomic(Atomic::Integer(Integer::I32))
                                            }
                                        ],
                                        output: Some(Type::Compound("Self".into()))
                                    }
                                    )
                                ]
                            }
                        ]
                    }
                ]
            }
        );
    }
}
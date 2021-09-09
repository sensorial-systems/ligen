//! Module representation.

mod import;
pub use import::*;

use crate::prelude::*;
use crate::ir::{Object, Path, Structure, Implementation, Visibility, Identifier, TypeDefinition, Enumeration, Attributes, Attribute};
use std::convert::TryFrom;
use std::collections::HashMap;
use std::io::Read;
use std::fs::File;

/// Module representation.
#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    /// Attributes.
    pub attributes: Attributes,
    /// Visibility.
    pub visibility: Visibility,
    /// Module name.
    pub name: Identifier,
    /// Imports.
    pub imports: Vec<Import>,
    /// Sub-modules.
    pub modules: Vec<Module>,
    /// Objects.
    pub objects: Vec<Object>
}

impl Module {
    /// Tells if ligen is ignoring this module.
    pub fn ignored(&self) -> bool {
        Self::ignored_from_attributes(&self.attributes)
    }

    fn ignored_from_attributes(attributes: &Attributes) -> bool {
        attributes.contains(&Attribute::Group("ligen".into(), Attribute::Group("ignore".into(), Default::default()).into()))
    }
}

impl TryFrom<&std::path::Path> for Module {
    type Error = Error;
    fn try_from(from: &std::path::Path) -> Result<Self> {
        // FIXME: This function needs a cleanup.
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
        let attributes = Module::parse_ligen_attributes(&file.attrs, &file.items)?;
        let ignored = Module::ignored_from_attributes(&attributes);
        let (modules, objects) = extract_modules_and_objects(ignored, &file.items, base_path.as_path())?;
        let imports = Imports::try_from(file.items.as_slice())?.0;
        Ok(Module { attributes, imports, visibility, name, modules, objects })
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

// FIXME: Find a better place for this function.
fn extract_modules_and_objects(ignored: bool, items: &[syn::Item], base_path: &std::path::Path) -> Result<(Vec<Module>, Vec<Object>)> {
    if ignored {
        Ok((Default::default(), Default::default()))
    } else {
        let modules = Module::parse_modules(&items, base_path)?;
        let objects = Module::parse_objects(&items)?;
        Ok((modules, objects))
    }
}

impl Module {
    fn parse_modules(items: &[syn::Item], base_path: &std::path::Path) -> Result<Vec<Module>> {
        let mut modules = Vec::new();
        for item in items {
            match item {
                syn::Item::Mod(module) => {
                    let module = Module::try_from((module.clone(), base_path))?;
                    if !module.ignored() {
                        modules.push(module)
                    }
                },
                _ => ()
            }
        }
        Ok(modules)
    }

    fn parse_ligen_attributes(attrs: &Vec<syn::Attribute>, items: &[syn::Item]) -> Result<Attributes> {
        let mut attributes: Attributes = attrs.clone().try_into()?;
        for item in items {
            match item {
                syn::Item::Macro(call) => {
                    let attribute = Attribute::try_from(call.clone())?;
                    if let Attribute::Group(identifier, grouped_attributes) = &attribute {
                        if *identifier == Identifier::from("inner_ligen") {
                            attributes.attributes.push(Attribute::Group("ligen".into(), grouped_attributes.clone()));
                        }
                    }
                },
                _ => ()
            }
        }
        Ok(attributes)
    }

    fn parse_objects(items: &[syn::Item]) -> Result<Vec<Object>> {
        let mut objects: HashMap<Path, (Option<TypeDefinition>, Vec<Implementation>)> = HashMap::new();
        for item in items {
            match item {
                syn::Item::Enum(enumeration) => {
                    let enumeration = Enumeration::try_from(enumeration.clone())?;
                    let path = enumeration.identifier.clone().into();
                    let definition = Some(TypeDefinition::Enumeration(enumeration));
                    if let Some((optional_definition, _)) = objects.get_mut(&path) {
                        *optional_definition = definition;
                    } else {
                        objects.insert(path, (definition, Default::default()));
                    }
                },
                syn::Item::Struct(structure) => {
                    let structure = Structure::try_from(structure.clone())?;
                    let path = structure.identifier.clone().into();
                    let definition = Some(TypeDefinition::Structure(structure));
                    if let Some((optional_definition, _implementations)) = objects.get_mut(&path) {
                        *optional_definition = definition;
                    } else {
                        objects.insert(path, (definition, Default::default()));
                    }
                },
                syn::Item::Impl(implementation) => {
                    // TODO: Consider `impl Trait for Object`?
                    if implementation.trait_.is_none() {
                        let implementation = Implementation::try_from(implementation.clone())?;
                        let path = implementation.self_.path();
                        if let Some((_definition, implementations)) = objects.get_mut(&path) {
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
            .map(|(path, (definition, implementations))| Object {
                definition: definition.expect(&format!("Type definition for {} not found.", path)),
                path,
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
            let attributes = Module::parse_ligen_attributes(&module.attrs, &items)?;
            let ignored = Module::ignored_from_attributes(&attributes);
            let (modules, objects) = extract_modules_and_objects(ignored, &items, base_path.as_path())?;
            let name = module.ident.into();
            let visibility = module.vis.into();
            let imports = Imports::try_from(items.as_slice())?.0;
            Ok(Self { attributes, visibility, name, imports, modules, objects })
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
    use crate::ir::{Object, Atomic, Integer, Type, Visibility, Function, Structure, Parameter, Implementation, ImplementationItem, Field, Attribute};
    use quote::quote;

    #[test]
    fn object() {
        let module = quote! {
            #[ligen(attribute)]
            mod objects {
                inner_ligen!(another_attribute);

                pub struct Object {
                    pub integer: i32
                }

                impl Object {
                    pub fn new(integer: i32) -> Self {
                        Self { integer }
                    }
                }

                pub struct AnotherObject;
            }
        };

        let module: syn::ItemMod = syn::parse2(module).expect("Couldn't parse.");
        assert_eq!(
            Module::try_from((module, std::path::Path::new(""))).expect("Failed to convert from ItemImpl"),
            Module {
                attributes: vec![
                    Attribute::Group("ligen".into(), Attribute::Group("attribute".into(), Default::default()).into()),
                    Attribute::Group("ligen".into(), Attribute::Group("another_attribute".into(), Default::default()).into()),
                ].into(),
                visibility: Visibility::Inherited,
                name: "objects".into(),
                imports: Default::default(),
                modules: Default::default(),
                objects: vec![
                    Object {
                        path: "AnotherObject".into(),
                        definition: TypeDefinition::Structure(Structure {
                            attributes: Default::default(),
                            visibility: Visibility::Public,
                            identifier: "AnotherObject".into(),
                            fields: Default::default(),
                        }),
                        implementations: Default::default()
                    },
                    Object {
                        path: "Object".into(),
                        definition: TypeDefinition::Structure(Structure {
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
                                                attributes: Default::default(),
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

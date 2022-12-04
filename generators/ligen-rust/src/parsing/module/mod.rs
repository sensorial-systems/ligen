//! Module representation.

mod import;

use crate::prelude::*;
use crate::{Object, Structure, Visibility, Identifier, Enumeration, Attributes, Attribute, Function, Module};
use syn::parse_quote::parse;
use std::path::PathBuf;
use ligen_ir::conventions::naming::{NamingConvention, SnakeCase};

fn extract_functions(items: &[syn::Item]) -> Vec<Function> {
    let mut functions = Vec::new();
    for item in items {
        if let syn::Item::Fn(function) = item {
            functions.push(SynItemFn(function.clone()).into());
        }
    }
    functions
}

// FIXME: Find a better place for this function.
fn extract_modules_and_objects(ignored: bool, visitor: &ModuleConversionHelper) -> Result<(Vec<Module>, Vec<Object>)> {
    if ignored {
        Ok((Default::default(), Default::default()))
    } else {
        let modules = parse_modules(visitor)?;
        let objects = if let Some(items) = &visitor.items {
            parse_objects(items)?
        } else {
            Default::default()
        };
        Ok((modules, objects))
    }
}

fn parse_modules(visitor: &ModuleConversionHelper) -> Result<Vec<Module>> {
    let mut modules = Vec::new();
    if let Some(items) = &visitor.items {
        for item in items {
            match item {
                syn::Item::Mod(module) => {
                    let visitor = ModuleConversionHelper::try_from((visitor, module))?;
                    let module = Module::try_from(visitor)?;
                    if !module.ignored() {
                        modules.push(module)
                    }
                },
                _ => ()
            }
        }
    }
    Ok(modules)
}

fn parse_ligen_attributes(attributes: &Attributes, items: &[syn::Item]) -> Result<Attributes> {
    let mut attributes = attributes.clone();
    for item in items {
        match item {
            syn::Item::Macro(call) => {
                let attribute = Attribute::try_from(SynItemMacro(call.clone()))?;
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
    let mut objects = Vec::new();
    for item in items {
        match item {
            syn::Item::Enum(enumeration) => {
                let enumeration = Enumeration::try_from(SynItemEnum(enumeration.clone()))?;
                objects.push(Object::from(enumeration));
            },
            syn::Item::Struct(structure) => {
                let structure = Structure::try_from(SynItemStruct(structure.clone()))?;
                objects.push(Object::from(structure));
            },
            // syn::Item::Impl(_implementation) => {
            //     // TODO: Consider `impl Trait for Object`?
            //     if implementation.trait_.is_none() {
            //         let mut implementation = Implementation::try_from(SynItemImpl(implementation.clone()))?;
            //         let path = implementation.self_.path();
            //         if let Some((_definition, existing_implementation)) = objects.get_mut(&path) {
            //             if let Some(existing_implementation) = existing_implementation {
            //                 existing_implementation.attributes.attributes.append(&mut implementation.attributes);
            //                 existing_implementation.items.append(&mut implementation.items);
            //             } else {
            //                 *existing_implementation = Some(implementation);
            //             }
            //         } else {
            //             objects.insert(path, (None, Some(implementation)));
            //         }
            //     }
            // }
            _ => ()
        }
    }
    Ok(objects)
}

#[allow(unused_qualifications)]
impl TryFrom<ProcMacro2TokenStream> for Module {
    type Error = Error;
    fn try_from(ProcMacro2TokenStream(tokenstream): ProcMacro2TokenStream) -> Result<Self> {
        let module = parse::<syn::ItemMod>(tokenstream);
        let directory = PathBuf::from("");
        let name = NamingConvention::SnakeCase(Default::default());
        let project = ProjectInfo { directory, name };
        let attributes = (LigenAttributes::try_from(module.attrs)?).into();
        let visibility = SynVisibility(module.vis).into();
        let identifier = Identifier::from(SynIdent(module.ident));
        let relative_path = PathBuf::from(identifier.name.clone());
        let items = module.content.clone().unwrap_or_default().1.into();
        ModuleConversionHelper { project, items, identifier, relative_path, visibility, attributes }.try_into()
    }
}

#[derive(Clone)]
pub struct ProjectInfo {
    pub directory: PathBuf,
    pub name: NamingConvention
}

struct ModuleConversionHelper {
    attributes: Attributes,
    items: Option<Vec<syn::Item>>,
    visibility: Visibility,
    identifier: Identifier,
    relative_path: PathBuf,
    project: ProjectInfo
}

impl TryFrom<(&ModuleConversionHelper, &syn::ItemMod)> for ModuleConversionHelper {
    type Error = Error;
    fn try_from(from: (&ModuleConversionHelper, &syn::ItemMod)) -> Result<Self> {
        let (visitor, module) = from;
        let project = visitor.project.clone();
        let visibility = SynVisibility(module.vis.clone()).into();
        let items = module.content.clone().map(|(_, content)| content).into();
        let attributes = (LigenAttributes::try_from(module.attrs.clone())?).into();
        let identifier = Identifier::from(SynIdent(module.ident.clone()));
        let relative_path = visitor.relative_path.join(identifier.name.clone());
        Ok(Self { visibility, items, attributes, relative_path, project, identifier })
    }
}

impl TryFrom<ProjectInfo> for Module {
    type Error = Error;
    fn try_from(project: ProjectInfo) -> Result<Self> {
        ModuleConversionHelper::try_from(project)?.try_into()
    }
}

impl TryFrom<ProjectInfo> for ModuleConversionHelper {
    type Error = Error;
    fn try_from(project: ProjectInfo) -> Result<Self> {
        let module_path = project.directory.join("src").join("lib.rs");
        let src = std::fs::read_to_string(module_path)?;
        let file = syn::parse_file(&src).map_err(|e| Error::Generic(Box::new(e)))?;
        let visibility = Visibility::Public;
        let items = Some(file.items);
        let attributes = (LigenAttributes::try_from(file.attrs)?).into();
        let identifier = SnakeCase::from(project.name.clone()).into();
        let relative_path = PathBuf::from("");
        Ok(ModuleConversionHelper { visibility, identifier, items, project, relative_path, attributes })
    }
}

impl TryFrom<ModuleConversionHelper> for Module {
    type Error = Error;
    fn try_from(visitor: ModuleConversionHelper) -> Result<Self> {
        let module = if let Some(items) = &visitor.items {
            let attributes = parse_ligen_attributes(&visitor.attributes, &items)?;
            let ignored = attributes.has_ignore_attribute();
            let (modules, objects) = extract_modules_and_objects(ignored, &visitor)?;
            let visibility = visitor.visibility;
            let imports = LigenImports::try_from(items.as_slice())?.0.0;
            let functions = extract_functions(items.as_slice());
            let path = visitor.identifier.into();
            Self { attributes, visibility, path, imports, modules, functions, objects }
        } else {
            // FIXME: Clean this up. This code is duplicated and can be simplified.
            let module_path = visitor.project.directory.join("src").join(visitor.relative_path);
            let src = if let Ok(src) = std::fs::read_to_string(module_path.with_extension("rs")) {
                src
            } else {
                std::fs::read_to_string(module_path.join("mod.rs"))?
            };
            let file = syn::parse_file(&src).map_err(|e| Error::Generic(Box::new(e)))?;
            let visibility = visitor.visibility;
            let items = Some(file.items);
            let attributes = (LigenAttributes::try_from(file.attrs)?).into();
            let identifier = visitor.identifier.clone();
            let relative_path = PathBuf::from(identifier.name.clone());
            let project = visitor.project.clone();
            ModuleConversionHelper { visibility, identifier, items, project, relative_path, attributes }.try_into()?
        };
        Ok(module)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Visibility, Structure};
    use quote::quote;
    use pretty_assertions::assert_eq;
    use ligen_ir::{Constant, Integer};

    #[test]
    fn module_file() -> Result<()> {
        let module = quote! { mod module; };
        let module = Module::try_from(ProcMacro2TokenStream(module))?;
        let expected_module = Module { path: "module".into(), ..Default::default() };
        assert_eq!(module, expected_module);
        Ok(())
    }

    #[test]
    fn module_path() -> Result<()> {
        let module = quote! {
            mod root {
                mod branch {
                    mod leaf {}
                }
            }
        };
        let mut module = Module::try_from(ProcMacro2TokenStream(module))?;
        module.guarantee_absolute_paths();
        let expected_module = Module {
            path: "root".into(),
            modules: vec![
                Module {
                    path: "root::branch".into(),
                    modules: vec![
                        Module {
                            path: "root::branch::leaf".into(),
                            ..Default::default()
                        }
                    ],
                    ..Default::default()
                }
            ],
            ..Default::default()
        };
        assert_eq!(module, expected_module);
        Ok(())
    }

    #[test]
    fn module_objects_with_methods() -> Result<()> {
        let module = quote! {
            pub mod types {
                pub struct Type;
                impl Type {
                    const Value: i32 = 123;
                    fn static_function() {}
                    fn method(&self) {}
                }
            }
        };
        let module = Module::try_from(ProcMacro2TokenStream(module))?;
        let expected_module = Module {
            path: "types".into(),
            visibility: Visibility::Public,
            objects: vec![
                Object {
                    constants: vec![
                        Constant {
                            identifier: "Value".into(),
                            type_: Integer::I32.into(),
                            literal: 123.into()
                        }
                    ],
                    functions: Default::default(),
                    methods: Default::default(),
                    definition: Structure {
                        attributes: Default::default(),
                        visibility: Visibility::Public,
                        fields: Default::default(),
                        path: "Type".into()
                    }.into()
                }
            ],
            ..Default::default()
        };
        assert_eq!(module, expected_module);
        Ok(())
    }

    #[test]
    fn module_objects() -> Result<()> {
        let module = quote! {
            pub mod types {
                pub struct Type;
            }
        };
        let module = Module::try_from(ProcMacro2TokenStream(module))?;
        let expected_module = Module {
            path: "types".into(),
            visibility: Visibility::Public,
            objects: vec![
                Object::from(Structure {
                        attributes: Default::default(),
                        visibility: Visibility::Public,
                        fields: Default::default(),
                        path: "Type".into()
                })
            ],
            ..Default::default()
        };
        assert_eq!(module, expected_module);
        Ok(())
    }

    #[test]
    fn object_finder() -> Result<()> {
        let module = quote! {
            pub mod types {
                pub struct Type;
            }
        };
        let module = Module::try_from(ProcMacro2TokenStream(module))?;
        let object = module.find_object(&"Type".into());
        let definition = quote! { pub struct Type; };
        let definition = Structure::try_from(ProcMacro2TokenStream(definition))?;
        let expected_object = Some(Object::from(definition));
        assert_eq!(object, expected_object.as_ref());
        Ok(())
    }
}

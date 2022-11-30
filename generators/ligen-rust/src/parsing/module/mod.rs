//! Module representation.

mod import;

use crate::prelude::*;
use crate::{Object, Path, Structure, Implementation, Visibility, Identifier, TypeDefinition, Enumeration, Attributes, Attribute, Function, Module, ProjectInfo};
use std::collections::HashMap;
use syn::parse_quote::parse;
use std::path::PathBuf;
use ligen_ir::conventions::naming::{NamingConvention, SnakeCase};

// TODO: This is a convertion between two types in ligen-ir which requires a new type, it indicates that there is a conceptual problem here.
impl TryFrom<LigenProjectInfo> for Module {
    type Error = Error;
    fn try_from(LigenProjectInfo(project): LigenProjectInfo) -> Result<Self> {
        ModuleConversionHelper::try_from(project)?.try_into()
    }
}

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
    let mut objects: HashMap<Path, (Option<TypeDefinition>, Option<Implementation>)> = HashMap::new();
    for item in items {
        match item {
            syn::Item::Enum(enumeration) => {
                let enumeration = Enumeration::try_from(SynItemEnum(enumeration.clone()))?;
                let path = enumeration.path.clone();
                let definition = Some(TypeDefinition::Enumeration(enumeration));
                if let Some((optional_definition, _)) = objects.get_mut(&path) {
                    *optional_definition = definition;
                } else {
                    objects.insert(path, (definition, None));
                }
            },
            syn::Item::Struct(structure) => {
                let structure = Structure::try_from(SynItemStruct(structure.clone()))?;
                let path = structure.path.clone();
                let definition = Some(TypeDefinition::Structure(structure));
                if let Some((optional_definition, _implementations)) = objects.get_mut(&path) {
                    *optional_definition = definition;
                } else {
                    objects.insert(path, (definition, None));
                }
            },
            syn::Item::Impl(implementation) => {
                // TODO: Consider `impl Trait for Object`?
                if implementation.trait_.is_none() {
                    let mut implementation = Implementation::try_from(SynItemImpl(implementation.clone()))?;
                    let path = implementation.self_.path();
                    if let Some((_definition, existing_implementation)) = objects.get_mut(&path) {
                        if let Some(existing_implementation) = existing_implementation {
                            existing_implementation.attributes.attributes.append(&mut implementation.attributes);
                            existing_implementation.items.append(&mut implementation.items);
                        } else {
                            *existing_implementation = Some(implementation);
                        }
                    } else {
                        objects.insert(path, (None, Some(implementation)));
                    }
                }
            }
            _ => ()
        }
    }
    let mut objects: Vec<_> = objects
        .into_iter()
        .filter_map(|(_, (definition, implementation))|
            if let (Some(definition), Some(implementation)) = (definition, implementation) {
                Some(Object { definition, implementation })
            } else {
                None
            }
        ).collect();
    // We sort it for consistency. HashMap doesn't guarantee any order.
    objects.sort_by(|a, b| a.definition.path().cmp(b.definition.path()));
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

struct ModuleConversionHelper {
    attributes: Attributes,
    items: Option<Vec<syn::Item>>,
    visibility: Visibility,
    identifier: Identifier,
    relative_path: PathBuf,
    project: ProjectInfo
}

// TODO: Is it safe to be removed?
// impl ModuleConversionHelper {
//     pub fn child(&self, attributes: Attributes, visibility: Visibility, items: Option<Vec<syn::Item>>, identifier: Identifier) -> Self {
//         let relative_path = self.relative_path.join(identifier.name.clone());
//         let project = self.project.clone();
//         Self { attributes, visibility, items, identifier, relative_path, project }
//     }
// }

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
            let visibility = Visibility::Public;
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
    fn type_definition_finder() -> Result<()> {
        let module = quote! {
            pub mod types {
                pub struct Type;

                pub fn process(type_: Type) {}
            }
        };
        let module = Module::try_from(ProcMacro2TokenStream(module))?;
        let definition = module.find_definition(&Path::from("Type"));
        let expected_definition = Some(
            TypeDefinition::Structure(
                Structure {
                    attributes: Default::default(),
                    visibility: Visibility::Public,
                    path: "Type".into(),
                    fields: Default::default()
                }
            )
        );
        assert_eq!(definition, expected_definition);
        Ok(())
    }
}

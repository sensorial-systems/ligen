//! Module representation.

mod import;

use crate::prelude::*;
use crate::{Object, Structure, Visibility, Identifier, Enumeration, Attributes, Attribute, Function, Module};
use syn::parse_quote::parse;
use std::path::PathBuf;
use syn::ImplItem;
use ligen_ir::conventions::naming::{NamingConvention, SnakeCase};
use ligen_ir::{Path, Project};

fn extract_functions(items: &[syn::Item]) -> Vec<Function> {
    let mut functions = Vec::new();
    for item in items {
        if let syn::Item::Fn(function) = item {
            functions.push(SynItemFn(function.clone()).into());
        }
    }
    functions
}

fn extract_modules(ignored: bool, visitor: &ModuleConversionHelper) -> Result<Vec<Module>> {
    let mut modules = Vec::new();
    if !ignored {
        for visitor in visitor.get_children()? {
            let module = Module::try_from(visitor)?;
            if !module.ignored() {
                modules.push(module)
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

fn extract_object_definitions(ignored: bool, visitor: &ModuleConversionHelper) -> Result<Vec<Object>> {
    let mut objects = Vec::new();
    if let (false, Some(items)) = (ignored, &visitor.items) {
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
                syn::Item::Type(_type) => {
                    todo!("Type object isn't implemented yet.")
                },
                syn::Item::Union(_union) => {
                    todo!("Union object isn't implemented yet.")
                },
                _ => ()
            }
        }
    }
    Ok(objects)
}

// FIXME: Make it private.
pub fn extract_object_implementations(project: &mut Project, ignored: bool, visitor: &ModuleConversionHelper) -> Result<()> {
    if let (false, Some(items)) = (ignored, &visitor.items) {
        for item in items {
            match item {
                syn::Item::Impl(implementation) => {
                    // TODO: Consider `impl Trait for Object`?
                    if implementation.trait_.is_none() {
                        if let syn::Type::Path(syn::TypePath { path, .. }) = &*implementation.self_ty {
                            // FIXME: Transform relative path to absolute path.
                            let path = Path::from(SynPath(path.clone()));
                            if let Some(object) = project.root_module.find_object_mut(&path) {
                                // TODO: Parse attributes and merge them with individual items.
                                // let attributes = implementation.attrs;
                                for item in &implementation.items {
                                    match item {
                                        ImplItem::Const(constant) => {
                                            let constant = SynImplItemConst(constant.clone()).into();
                                            object.constants.push(constant)
                                        },
                                        ImplItem::Method(method) => {
                                            if method.sig.receiver().is_some() {
                                                let method = SynImplItemMethod(method.clone()).into();
                                                object.methods.push(method)
                                            } else {
                                                let function = SynImplItemMethod(method.clone()).into();
                                                object.functions.push(function)
                                            }
                                        }
                                        _ => ()
                                    }
                                }
                            }
                        }
                    }
                }
                _ => ()
            }
        }
    }
    for visitor in visitor.get_children()? {
        // FIXME: ignored is hardcoded.
        extract_object_implementations(project, false, &visitor)?;
    }
    Ok(())
}

impl TryFrom<ProcMacro2TokenStream> for ModuleConversionHelper {
    type Error = Error;
    fn try_from(ProcMacro2TokenStream(token_stream): ProcMacro2TokenStream) -> Result<Self> {
        let module = parse::<syn::ItemMod>(token_stream);
        let directory = PathBuf::from("");
        let name = NamingConvention::SnakeCase(Default::default());
        let project = ProjectInfo { directory, name };
        let attributes = (LigenAttributes::try_from(module.attrs)?).into();
        let visibility = SynVisibility(module.vis).into();
        let identifier = Identifier::from(SynIdent(module.ident));
        let relative_path = PathBuf::from(identifier.name.clone());
        let items = module.content.clone().unwrap_or_default().1.into();
        Ok(ModuleConversionHelper { project, items, identifier, relative_path, visibility, attributes })
    }
}

#[allow(unused_qualifications)]
impl TryFrom<ProcMacro2TokenStream> for Module {
    type Error = Error;
    fn try_from(token_stream: ProcMacro2TokenStream) -> Result<Self> {
        ModuleConversionHelper::try_from(token_stream)?.try_into()
    }
}

#[derive(Clone)]
pub struct ProjectInfo {
    pub directory: PathBuf,
    pub name: NamingConvention
}

// FIXME: This should be private.
pub struct ModuleConversionHelper {
    attributes: Attributes,
    items: Option<Vec<syn::Item>>,
    visibility: Visibility,
    identifier: Identifier,
    relative_path: PathBuf,
    project: ProjectInfo
}

impl ModuleConversionHelper {
    fn get_children(&self) -> Result<Vec<ModuleConversionHelper>> {
        let mut modules = Vec::new();
        if let Some(items) = &self.items {
            for item in items {
                match item {
                    syn::Item::Mod(module) => {
                        modules.push(ModuleConversionHelper::try_from((self, module))?);
                    },
                    _ => ()
                }
            }
        }
        Ok(modules)
    }
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
            let modules = extract_modules(ignored, &visitor)?;
            let objects = extract_object_definitions(ignored, &visitor)?;
            let visibility = visitor.visibility;
            let imports = LigenImports::try_from(items.as_slice())?.0.0;
            let functions = extract_functions(items.as_slice());
            let path = visitor.identifier.into();
            Self { attributes, visibility, path, imports, modules, functions, objects }
        } else {
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

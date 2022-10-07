//! Module representation.

mod import;

use crate::prelude::*;
use crate::{Object, Path, Structure, Implementation, Visibility, Identifier, TypeDefinition, Enumeration, Attributes, Attribute, Function, Module, ProjectInfo};
use std::collections::HashMap;
use syn::parse_quote::parse;
use std::path::PathBuf;
use ligen_utils::conventions::naming::NamingConvention;

// TODO: This is a convertion between two types in ligen-ir, it indicates that there is a conceptual problem here.
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
    let mut objects: HashMap<Path, (Option<TypeDefinition>, Vec<Implementation>)> = HashMap::new();
    for item in items {
        match item {
            syn::Item::Enum(enumeration) => {
                let enumeration = Enumeration::try_from(SynItemEnum(enumeration.clone()))?;
                let path = enumeration.identifier.clone().into();
                let definition = Some(TypeDefinition::Enumeration(enumeration));
                if let Some((optional_definition, _)) = objects.get_mut(&path) {
                    *optional_definition = definition;
                } else {
                    objects.insert(path, (definition, Default::default()));
                }
            },
            syn::Item::Struct(structure) => {
                let structure = Structure::try_from(SynItemStruct(structure.clone()))?;
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
                    let implementation = Implementation::try_from(SynItemImpl(implementation.clone()))?;
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
            // FIXME: This shouldn't use expect
            definition: definition.expect(&format!("Type definition for {} not found.", (&path as &dyn Display))),
            path,
            implementations
        })
        .collect();
    objects.sort_by(|a, b| a.path.cmp(&b.path));
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
        let file = syn::parse_file(&src)?;
        let visibility = Visibility::Public;
        let items = Some(file.items);
        let attributes = (LigenAttributes::try_from(file.attrs)?).into();
        let identifier = Identifier::from("crate"); // FIXME: Rusty
        let relative_path = PathBuf::from("");
        Ok(ModuleConversionHelper { visibility, identifier, items, project, relative_path, attributes })
    }
}

impl TryFrom<ModuleConversionHelper> for Module {
    type Error = Error;
    fn try_from(visitor: ModuleConversionHelper) -> Result<Self> {
        if let Some(items) = &visitor.items {
            let attributes = parse_ligen_attributes(&visitor.attributes, &items)?;
            let ignored = attributes.has_ignore_attribute();
            let (modules, objects) = extract_modules_and_objects(ignored, &visitor)?;
            let name = visitor.identifier;
            let visibility = visitor.visibility;
            let imports = LigenImports::try_from(items.as_slice())?.0.0;
            let functions = extract_functions(items.as_slice());
            let path = visitor.relative_path.into();
            Ok(Self { attributes, visibility, path, name, imports, modules, functions, objects })
        } else {
            // FIXME: Clean this up. This code is duplicated and can be simplified.
            let module_path = visitor.project.directory.join("src").join(visitor.relative_path);
            let src = if let Ok(src) = std::fs::read_to_string(module_path.with_extension("rs")) {
                src
            } else {
                std::fs::read_to_string(module_path.join("mod.rs"))?
            };
            let file = syn::parse_file(&src)?;
            let visibility = Visibility::Public;
            let items = Some(file.items);
            let attributes = (LigenAttributes::try_from(file.attrs)?).into();
            let identifier = visitor.identifier.clone();
            let relative_path = PathBuf::from(identifier.name.clone());
            let project = visitor.project.clone();
            ModuleConversionHelper { visibility, identifier, items, project, relative_path, attributes }.try_into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Object, Atomic, Integer, Type, Visibility, Function, Structure, Parameter, Implementation, ImplementationItem, Field, Attribute, Method, Mutability};
    use quote::quote;
    use pretty_assertions::assert_eq;

    #[test]
    fn imports() -> Result<()> {
        let module = quote! {
            mod root {
                mod object {
                    pub struct Object1;
                }
                mod objects {
                    pub struct Object2;
                    pub struct Object3;
                    struct Object4;
                    mod deeper {
                        pub struct Object5;
                        pub struct Object6;
                        struct Object7;
                    }
                    mod deeper2 {
                        pub struct Object8;
                        pub struct Object9;
                        pub struct ObjectA;
                    }
                    pub use deeper::*;
                    pub use deeper2::Object8;
                    use deeper2::Object9;
                    pub use deeper2::ObjectA as ObjectTen;
                }
                pub use object::Object1;
                pub use objects::*;
            }
        };
        let expected_module = quote! {
            mod root {
                mod object {
                    pub struct Object1;
                }
                mod objects {
                    pub struct Object2;
                    pub struct Object3;
                    struct Object4;
                    mod deeper {
                        pub struct Object5;
                        pub struct Object6;
                        struct Object7;
                    }
                    mod deeper2 {
                        pub struct Object8;
                        pub struct Object9;
                        pub struct ObjectA;
                    }
                    pub use deeper2::Object8;
                    use deeper2::Object9;
                    pub use deeper2::ObjectA as ObjectTen;
                    pub use deeper::Object5;
                    pub use deeper::Object6;
                }
                pub use object::Object1;
                pub use objects::Object2;
                pub use objects::Object3;
                pub use objects::Object8;
                pub use objects::ObjectTen;
                pub use objects::Object5;
                pub use objects::Object6;
            }
        };

        let expected_module = Module::try_from(ProcMacro2TokenStream(expected_module))?;
        let mut module = Module::try_from(ProcMacro2TokenStream(module))?;
        module.replace_wildcard_imports();
        assert_eq!(module, expected_module);
        Ok(())
    }

    #[test]
    fn object() -> Result<()> {
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

                mod deeper_module {

                }
            }
        };
        let module = Module::try_from(ProcMacro2TokenStream(module))?;
        assert_eq!(
            module,
            Module {
                attributes: vec![
                    Attribute::Group("ligen".into(), Attribute::Group("attribute".into(), Default::default()).into()),
                    Attribute::Group("ligen".into(), Attribute::Group("another_attribute".into(), Default::default()).into()),
                ].into(),
                visibility: Visibility::Inherited,
                path: Path::from("objects"),
                name: "objects".into(),
                imports: Default::default(),
                modules: vec! [
                    Module {
                        attributes: Default::default(),
                        visibility: Visibility::Inherited,
                        path: "objects::deeper_module".into(),
                        name: "deeper_module".into(),
                        imports: Default::default(),
                        modules: Default::default(),
                        functions: Default::default(),
                        objects: Default::default()
                    }
                ],
                functions: Default::default(),
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
                                    identifier: Some("integer".into()),
                                    type_: Type::Atomic(Atomic::Integer(Integer::I32))
                                }
                            ]
                        }),
                        implementations: vec![
                            Implementation {
                                attributes: Default::default(),
                                self_: Type::Compound("Object".into(), Default::default()),
                                items: vec![
                                    ImplementationItem::Method(Function {
                                        attributes: Default::default(),
                                        method: Some(Method {
                                            mutability: Mutability::Constant,
                                            owner: Type::from(Identifier::new("Object"))
                                        }),
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
                                        output: Some(Type::Compound("Self".into(), Default::default()))
                                    }
                                    )
                                ]
                            }
                        ]
                    }
                ]
            }
        );
        Ok(())
    }
}

//! Module representation.

mod import;

use syn::Item;
use ligen_ir::{Constant, Enumeration, Object, Path, Project, Structure};
use ligen_parsing::{Context, ParseFrom};
use crate::prelude::*;
use crate::{Identifier, Function, Module};

fn extract_functions(items: &[syn::Item]) -> Vec<Function> {
    let mut functions = Vec::new();
    for item in items {
        if let syn::Item::Fn(function) = item {
            functions.push(SynItemFn(function.clone()).into());
        }
    }
    functions
}

fn extract_modules(context: &Context<'_>, ignored: bool, items: Vec<syn::Item>) -> Result<Vec<Module>> {
    let mut modules = Vec::new();
    if !ignored {
        let items = items.into_iter().filter_map(|item| if let syn::Item::Mod(module) = item { Some(module) } else { None });
        for item in items {
            let module = Module::parse_from(context, SynItemMod(item))?;
            if !module.ignored() {
                modules.push(module)
            }
        }
    }
    Ok(modules)
}

// fn parse_ligen_attributes(attributes: &Attributes, items: &[syn::Item]) -> Result<Attributes> {
//     let mut attributes = attributes.clone();
//     for item in items {
//         match item {
//             syn::Item::Macro(call) => {
//                 let attribute = Attribute::try_from(SynItemMacro(call.clone()))?;
//                 if let Attribute::Group(identifier, grouped_attributes) = &attribute {
//                     if *identifier == Identifier::from("inner_ligen") {
//                         attributes.attributes.push(Attribute::Group("ligen".into(), grouped_attributes.clone()));
//                     }
//                 }
//             },
//             _ => ()
//         }
//     }
//     Ok(attributes)
// }

fn extract_object_definitions(ignored: bool, items: &[syn::Item]) -> Result<Vec<Object>> {
    let mut objects = Vec::new();
    if !ignored {
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
pub fn extract_object_implementations(project: &mut Project, ignored: bool, items: &[syn::Item]) -> Result<()> {
    if !ignored {
        for item in items {
            match item {
                syn::Item::Mod(module) => if let Some((_, items)) = &module.content {
                    // FIXME: Hardcoded ignored.
                    extract_object_implementations(project, false, items.as_slice())?;
                },
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
                                        syn::ImplItem::Const(constant) => {
                                            let constant = SynImplItemConst(constant.clone()).try_into()?;
                                            object.constants.push(constant)
                                        },
                                        syn::ImplItem::Method(method) => {
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
    Ok(())
}

impl ParseFrom<ProcMacro2TokenStream> for Module {
    fn parse_from(context: &Context<'_>, ProcMacro2TokenStream(token_stream): ProcMacro2TokenStream) -> Result<Self> {
        let module = syn::parse2::<syn::ItemMod>(token_stream).map_err(|_e| "Failed to parse syn::ItemMod")?;
        Module::parse_from(context, SynItemMod(module))
    }
}

impl ParseFrom<SynItemMod> for Module {
    fn parse_from(context: &Context<'_>, SynItemMod(module): SynItemMod) -> Result<Self> {
        let items = module
            .content
            .map(|(_, items)| items)
            .ok_or("Module file isn't loaded.")?;
        let attributes = (LigenAttributes::try_from(module.attrs)?).into();
        let visibility = SynVisibility(module.vis).into();
        let path = Identifier::from(SynIdent(module.ident)).into();
        let imports = LigenImports::try_from(items.as_slice())?.0.0;
        let functions = extract_functions(items.as_slice());
        let objects = extract_object_definitions(false, items.as_slice())?;
        let constants = extract_constants(context, false, items.as_slice())?;
        let modules = extract_modules(context, false, items)?;
        Ok(Self { attributes, visibility, path, imports, functions, objects, constants, modules })
    }
}

fn extract_constants(_context: &Context, _: bool, items: &[Item]) -> Result<Vec<Constant>> {
    let mut constants = Vec::new();
    for item in items {
        if let syn::Item::Const(constant) = item {
            constants.push(SynItemConst(constant.clone()).try_into()?);
        }
    }
    Ok(constants)
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn module_file() -> Result<()> {
        // let module = quote! { mod module; };
        // let result = Module::parse(ProcMacro2TokenStream(module));
        // assert!(result.is_err()); // Module file isn't loaded.
        // Ok(())
        panic!("Not implemented yet.");
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
        // let mut module = Module::try_from(ProcMacro2TokenStream(module))?;
        // module.guarantee_absolute_paths();
        // let expected_module = Module {
        //     path: "root".into(),
        //     modules: vec![
        //         Module {
        //             path: "root::branch".into(),
        //             modules: vec![
        //                 Module {
        //                     path: "root::branch::leaf".into(),
        //                     ..Default::default()
        //                 }
        //             ],
        //             ..Default::default()
        //         }
        //     ],
        //     ..Default::default()
        // };
        // assert_eq!(module, expected_module);
        // Ok(())
        panic!("Not implemented yet.");
    }

    #[test]
    fn module_objects() -> Result<()> {
        let module = quote! {
            pub mod types {
                pub struct Type;
            }
        };
        // let module = Module::try_from(ProcMacro2TokenStream(module))?;
        // let expected_module = Module {
        //     path: "types".into(),
        //     visibility: Visibility::Public,
        //     objects: vec![
        //         Object::from(Structure {
        //                 attributes: Default::default(),
        //                 visibility: Visibility::Public,
        //                 fields: Default::default(),
        //                 path: "Type".into()
        //         })
        //     ],
        //     ..Default::default()
        // };
        // assert_eq!(module, expected_module);
        // Ok(())
        panic!("Not implemented yet.");
    }

    #[test]
    fn object_finder() -> Result<()> {
        let module = quote! {
            pub mod types {
                pub struct Type;
            }
        };
        // let module = Module::try_from(ProcMacro2TokenStream(module))?;
        // let object = module.find_object(&"Type".into());
        // let definition = quote! { pub struct Type; };
        // let definition = Structure::try_from(ProcMacro2TokenStream(definition))?;
        // let expected_object = Some(Object::from(definition));
        // assert_eq!(object, expected_object.as_ref());
        // Ok(())
        panic!("Not implemented yet.");
    }
}

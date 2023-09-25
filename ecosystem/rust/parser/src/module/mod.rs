//! Module representation.

mod import;

use ligen_ir::{Constant, Object, Project};
use ligen_parsing::Parser;
use crate::prelude::*;
use ligen_ir::{Function, Module};
use crate::constant::ConstantParser;
use crate::function::{FunctionParser, MethodParser};
use crate::identifier::IdentifierParser;
use crate::macro_attributes::attributes::AttributesParser;
use crate::module::import::ImportsParser;
use crate::path::PathParser;
use crate::types::enumeration::EnumerationParser;
use crate::types::structure::StructureParser;
use crate::visibility::VisibilityParser;

pub struct ModuleParser;

impl ModuleParser {
    fn extract_functions(items: &[syn::Item]) -> Result<Vec<Function>> {
        let mut functions = Vec::new();
        for item in items {
            if let syn::Item::Fn(function) = item {
                functions.push(FunctionParser.parse(function.clone())?);
            }
        }
        Ok(functions)
    }

    fn extract_modules(&self, ignored: bool, items: Vec<syn::Item>) -> Result<Vec<Module>> {
        let mut modules = Vec::new();
        if !ignored {
            let items = items.into_iter().filter_map(|item| if let syn::Item::Mod(module) = item { Some(module) } else { None });
            for item in items {
                let module = self.parse(item)?;
                if !module.ignored() {
                    modules.push(module)
                }
            }
        }
        Ok(modules)
    }

// TODO: Is it still useful?
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
                        let attributes = AttributesParser.parse(enumeration.attrs.clone())?;
                        let identifier = IdentifierParser.parse(enumeration.ident.clone())?;
                        let visibility = VisibilityParser.parse(enumeration.vis.clone())?;
                        let enumeration = EnumerationParser.parse(enumeration.clone())?;
                        objects.push(Object {
                            attributes,
                            identifier,
                            visibility,
                            definition: enumeration.into(),
                            .. Default::default()
                        });
                    },
                    syn::Item::Struct(structure) => {
                        let attributes = AttributesParser.parse(structure.attrs.clone())?;
                        let identifier = IdentifierParser.parse(structure.ident.clone())?;
                        let visibility = VisibilityParser.parse(structure.vis.clone())?;
                        let structure = StructureParser.parse(structure.clone())?;
                        objects.push(Object {
                            attributes,
                            identifier,
                            visibility,
                            definition: structure.into(),
                            .. Default::default()
                        });
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
                        Self::extract_object_implementations(project, false, items.as_slice())?;
                    },
                    syn::Item::Impl(implementation) => {
                        // TODO: Consider `impl Trait for Object`?
                        if implementation.trait_.is_none() {
                            if let syn::Type::Path(syn::TypePath { path, .. }) = &*implementation.self_ty {
                                // FIXME: Transform relative path to absolute path.
                                let path = PathParser.parse(path.clone())?;
                                if let Some(object) = project.root_module.find_object_mut(&path) {
                                    // TODO: Parse attributes and merge them with individual items.
                                    // let attributes = implementation.attrs;
                                    for item in &implementation.items {
                                        match item {
                                            syn::ImplItem::Const(constant) => {
                                                let constant = ConstantParser.parse(constant.clone())?;
                                                object.constants.push(constant)
                                            },
                                            syn::ImplItem::Method(method) => {
                                                if method.sig.receiver().is_some() {
                                                    let method = MethodParser.parse(method.clone())?;
                                                    object.methods.push(method)
                                                } else {
                                                    let function = FunctionParser.parse(method.clone())?;
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

    fn extract_constants(&self, _: bool, items: &[syn::Item]) -> Result<Vec<Constant>> {
        let mut constants = Vec::new();
        for item in items {
            if let syn::Item::Const(constant) = item {
                constants.push(ConstantParser.parse(constant.clone())?);
            }
        }
        Ok(constants)
    }

}

impl Parser<proc_macro2::TokenStream> for ModuleParser {
    type Output = Module;
    fn parse(&self, token_stream: proc_macro2::TokenStream) -> Result<Self::Output> {
        syn::parse2::<syn::ItemMod>(token_stream)
            .map_err(|e| Error::Message(format!("Failed to parse module: {:?}", e)))
            .and_then(|module| self.parse(module))
    }
}

impl Parser<syn::ItemMod> for ModuleParser {
    type Output = Module;
    fn parse(&self, module: syn::ItemMod) -> Result<Self::Output> {
        let items = module
            .content
            .map(|(_, items)| items)
            .ok_or("Module file isn't loaded.")?;
        let attributes = AttributesParser.parse(module.attrs)?;
        let visibility = VisibilityParser.parse(module.vis)?;
        let identifier = IdentifierParser.parse(module.ident)?;
        let imports = ImportsParser.parse(items.as_slice())?.0;
        let functions = Self::extract_functions(items.as_slice())?;
        let objects = Self::extract_object_definitions(false, items.as_slice())?;
        let constants = self.extract_constants( false, items.as_slice())?;
        let modules = self.extract_modules( false, items)?;
        Ok(Self::Output { attributes, visibility, identifier, imports, functions, objects, constants, modules })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use pretty_assertions::assert_eq;
    use ligen_ir::Visibility;

    #[test]
    fn module_file() -> Result<()> {
        let parser = ModuleParser;
        let module = quote! { mod module; };
        let result = parser.parse(module);
        assert!(result.is_err()); // Module file isn't loaded.
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
        let parser = ModuleParser;
        let module = parser.parse(module)?;
        let expected_module = Module {
            identifier: "root".into(),
            modules: vec![
                Module {
                    identifier: "branch".into(),
                    modules: vec![
                        Module {
                            identifier: "leaf".into(),
                            visibility: Visibility::Private,
                            ..Default::default()
                        }
                    ],
                    visibility: Visibility::Private,
                    ..Default::default()
                }
            ],
            visibility: Visibility::Private,
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
        let parser = ModuleParser;
        let module = parser.parse(module)?;
        let expected_module = Module {
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
        assert_eq!(module, expected_module);
        Ok(())
    }
}

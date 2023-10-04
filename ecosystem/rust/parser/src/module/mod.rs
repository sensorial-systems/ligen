//! Module representation.

mod import;

use syn::spanned::Spanned;
use ligen::ir::{Constant, Object};
use ligen::parsing::Parser;
use crate::prelude::*;
use ligen::ir::{Function, Module, Import};
use crate::constant::ConstantParser;
use crate::function::FunctionParser;
use crate::identifier::IdentifierParser;
use crate::macro_attributes::attributes::AttributesParser;
use crate::module::import::ImportsParser;
use crate::types::enumeration::EnumerationParser;
use crate::types::structure::StructureParser;
use crate::visibility::VisibilityParser;

pub struct ModuleParser;

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

        let imports = self.extract_imports(items.as_slice())?;
        let functions = self.extract_functions(items.as_slice())?;
        let objects = self.extract_object_definitions(false, items.as_slice())?;
        let constants = self.extract_constants(false, items.as_slice())?;
        let modules = self.extract_modules(false, items)?;
        Ok(Self::Output { attributes, visibility, identifier, imports, functions, objects, constants, modules })
    }
}

impl Parser<&std::path::Path> for ModuleParser {
    type Output = Module;
    fn parse(&self, path: &std::path::Path) -> Result<Self::Output> {
        let module = syn2::file_parser::parse_file_recursive(path)?;
        let ident = syn::Ident::new(path.file_stem().unwrap_or_default().to_str().unwrap_or_default(), module.span()); // FIXME: This is hardcoded.
        let attrs = module.attrs;
        let pub_token = Default::default();
        let semi = Default::default();
        let mod_token = Default::default();
        let content = Some((Default::default(), module.items));
        let vis = syn::Visibility::Public(syn::VisPublic { pub_token });
        let module = syn::ItemMod { attrs, vis, mod_token, ident, semi, content };
        self.parse(module)
    }
}

impl ModuleParser {
    fn extract_imports(&self, items: &[syn::Item]) -> Result<Vec<Import>> {
        let mut imports: Vec<Import> = Default::default();
        for item in items {
            if let syn::Item::Use(import) = item {
                imports.append(&mut ImportsParser.parse(import.clone())?);
            }
        }
        Ok(imports)
    }
    fn extract_functions(&self, items: &[syn::Item]) -> Result<Vec<Function>> {
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
            let items = items
                .into_iter()
                .filter_map(|item| {
                    if let syn::Item::Mod(module) = item {
                        Some(module)
                    } else {
                        None
                    }
                });
            for module in items {
                let module = self.parse(module)?;
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

    fn extract_object_definitions(&self, ignored: bool, items: &[syn::Item]) -> Result<Vec<Object>> {
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

    // FIXME: Implement it.
    // fn extract_object_implementations(project: &mut Project, ignored: bool, items: &[syn::Item]) -> Result<()> {
    //     if !ignored {
    //         for item in items {
    //             match item {
    //                 syn::Item::Mod(module) => if let Some((_, items)) = &module.content {
    //                     // FIXME: Hardcoded ignored.
    //                     Self::extract_object_implementations(project, false, items.as_slice())?;
    //                 },
    //                 syn::Item::Impl(implementation) => {
    //                     // TODO: Consider `impl Trait for Object`?
    //                     if implementation.trait_.is_none() {
    //                         if let syn::Type::Path(syn::TypePath { path, .. }) = &*implementation.self_ty {
    //                             // FIXME: Transform relative path to absolute path.
    //                             let path = PathParser.parse(path.clone())?;
    //                             if let Some(object) = project.root_module.find_object_mut(&path) {
    //                                 // TODO: Parse attributes and merge them with individual items.
    //                                 // let attributes = implementation.attrs;
    //                                 for item in &implementation.items {
    //                                     match item {
    //                                         syn::ImplItem::Const(constant) => {
    //                                             let constant = ConstantParser.parse(constant.clone())?;
    //                                             object.constants.push(constant)
    //                                         },
    //                                         syn::ImplItem::Method(method) => {
    //                                             if method.sig.receiver().is_some() {
    //                                                 let method = MethodParser.parse(method.clone())?;
    //                                                 object.methods.push(method)
    //                                             } else {
    //                                                 let function = FunctionParser.parse(method.clone())?;
    //                                                 object.functions.push(function)
    //                                             }
    //                                         }
    //                                         _ => ()
    //                                     }
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 }
    //                 _ => ()
    //             }
    //         }
    //     }
    //     Ok(())
    // }

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


#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use ligen::ir::module::mock;
    use ligen::parsing::assert::*;

    #[test]
    fn module_file() -> Result<()> {
        assert_failure(ModuleParser, quote! { mod module; })
    }

    #[test]
    fn sub_modules() -> Result<()> {
        assert_eq(ModuleParser, mock::sub_modules(), quote! {
            pub mod root {
                pub mod branch {
                    pub mod leaf {}
                }
            }
        })
    }

    #[test]
    fn module_objects() -> Result<()> {
        assert_eq(ModuleParser, mock::module_objects(), quote! {
            pub mod objects {
                pub struct Structure;
                pub enum Enumeration {}
                pub const CONSTANT: bool = false;
                pub fn function() {}
            }
        })
    }
}

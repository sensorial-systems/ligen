//! Module representation.

mod import;

use syn::spanned::Spanned;
use ligen::ir::Object;
use ligen::parsing::parser::Parser;
use crate::prelude::*;
use ligen::ir::{Function, Module, Import, TypeDefinition, Interface};
use crate::object::ObjectParser;
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
        let attributes = AttributesParser::default().parse(module.attrs)?;
        let visibility = VisibilityParser.parse(module.vis)?;
        let identifier = IdentifierParser::new().parse(module.ident)?;

        let imports = self.extract_imports(items.as_slice())?;
        let functions = self.extract_functions(items.as_slice())?;
        let objects = self.extract_objects(items.as_slice())?;
        let types = self.extract_types(items.as_slice())?;
        let interfaces = self.extract_interfaces(items.as_slice())?;
        let modules = self.extract_modules(items)?;
        Ok(Self::Output { attributes, visibility, identifier, imports, functions, objects, types, interfaces, modules })
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
    fn extract_interfaces(&self, _items: &[syn::Item]) -> Result<Vec<Interface>> {
        Ok(Default::default())
    }
    fn extract_types(&self, items: &[syn::Item]) -> Result<Vec<TypeDefinition>> {
        let mut types = Vec::new();
        for item in items {
            match item {
                syn::Item::Enum(enumeration) =>
                    types.push(EnumerationParser::new().parse(enumeration.clone())?),
                syn::Item::Struct(structure) =>
                    types.push(StructureParser::new().parse(structure.clone())?),
                syn::Item::Type(_type) => {
                    todo!("Type object isn't implemented yet.")
                },
                syn::Item::Union(_union) => {
                    todo!("Union object isn't implemented yet.")
                },
                _ => ()
            }
        }
        Ok(types)
    }

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

    fn extract_modules(&self, items: Vec<syn::Item>) -> Result<Vec<Module>> {
        let mut modules = Vec::new();
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
            modules.push(self.parse(module)?)
        }
        Ok(modules)
    }

    fn extract_objects(&self, items: &[syn::Item]) -> Result<Vec<Object>> {
        let mut objects = Vec::new();
        for item in items {
            if let syn::Item::Const(constant) = item {
                objects.push(ObjectParser.parse(constant.clone())?);
            }
        }
        Ok(objects)
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
    fn module_types() -> Result<()> {
        assert_eq(ModuleParser, mock::module_types(), quote! {
            pub mod types {
                pub struct Structure;
                pub enum Enumeration {}
            }
        })
    }

    // TODO: Implement these:
    // #[test]
    // fn module_functions() -> Result<()> {
    //     // pub fn function() {}
    //
    //     todo!()
    // }
    //
    // #[test]
    // fn module_constants() -> Result<()> {
    //     // pub const CONSTANT: bool = false;
    //     todo!()
    // }
    //
    // #[test]
    // fn module_interfaces() -> Result<()> {
    //     // pub trait Interface {
    //     // }
    //     todo!()
    // }
}

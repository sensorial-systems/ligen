//! Module representation.

mod import;

use syn::spanned::Spanned;
use ligen::ir::Object;
use ligen::parser::{Parser, ParserConfig};
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
    fn parse(&self, token_stream: proc_macro2::TokenStream, config: &ParserConfig) -> Result<Self::Output> {
        syn::parse2::<syn::ItemMod>(token_stream)
            .map_err(|e| Error::Message(format!("Failed to parse module: {:?}", e)))
            .and_then(|module| self.parse(module, config))
    }
}

impl Parser<syn::ItemMod> for ModuleParser {
    type Output = Module;
    fn parse(&self, module: syn::ItemMod, config: &ParserConfig) -> Result<Self::Output> {
        let items = module
            .content
            .map(|(_, items)| items)
            .ok_or("Module file isn't loaded.")?;
        let attributes = AttributesParser::default().parse(module.attrs, config)?;
        let visibility = VisibilityParser.parse(module.vis, config)?;
        let identifier = IdentifierParser::new().parse(module.ident, config)?;

        let imports = self.extract_imports(items.as_slice(), config)?;
        let functions = self.extract_functions(items.as_slice(), config)?;
        let objects = self.extract_objects(items.as_slice(), config)?;
        let types = self.extract_types(items.as_slice(), config)?;
        let interfaces = self.extract_interfaces(items.as_slice())?;
        let modules = self.extract_modules(items, config)?;
        Ok(Self::Output { attributes, visibility, identifier, imports, functions, objects, types, interfaces, modules })
    }
}

impl Parser<&std::path::Path> for ModuleParser {
    type Output = Module;
    fn parse(&self, path: &std::path::Path, config: &ParserConfig) -> Result<Self::Output> {
        let module = syn2::file_parser::parse_file_recursive(path)?;
        let ident = syn::Ident::new(path.file_stem().unwrap_or_default().to_str().unwrap_or_default(), module.span()); // FIXME: This is hardcoded.
        let attrs = module.attrs;
        let pub_token = Default::default();
        let semi = Default::default();
        let mod_token = Default::default();
        let content = Some((Default::default(), module.items));
        let vis = syn::Visibility::Public(pub_token);
        let unsafety = Default::default();
        let module = syn::ItemMod { unsafety, attrs, vis, mod_token, ident, semi, content };
        self.parse(module, config)
    }
}

impl ModuleParser {
    fn extract_interfaces(&self, _items: &[syn::Item]) -> Result<Vec<Interface>> {
        Ok(Default::default())
    }
    fn extract_types(&self, items: &[syn::Item], config: &ParserConfig) -> Result<Vec<TypeDefinition>> {
        let mut types = Vec::new();
        for item in items {
            match item {
                syn::Item::Enum(enumeration) =>
                    types.push(EnumerationParser::new().parse(enumeration.clone(), config)?),
                syn::Item::Struct(structure) =>
                    types.push(StructureParser::new().parse(structure.clone(), config)?),
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

    fn extract_imports(&self, items: &[syn::Item], config: &ParserConfig) -> Result<Vec<Import>> {
        let mut imports: Vec<Import> = Default::default();
        for item in items {
            if let syn::Item::Use(import) = item {
                imports.append(&mut ImportsParser.parse(import.clone(), config)?);
            }
        }
        Ok(imports)
    }
    fn extract_functions(&self, items: &[syn::Item], config: &ParserConfig) -> Result<Vec<Function>> {
        let mut functions = Vec::new();
        for item in items {
            if let syn::Item::Fn(function) = item {
                functions.push(FunctionParser.parse(function.clone(), config)?);
            }
        }
        Ok(functions)
    }

    fn extract_modules(&self, items: Vec<syn::Item>, config: &ParserConfig) -> Result<Vec<Module>> {
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
            modules.push(self.parse(module, config)?)
        }
        Ok(modules)
    }

    fn extract_objects(&self, items: &[syn::Item], config: &ParserConfig) -> Result<Vec<Object>> {
        let mut objects = Vec::new();
        for item in items {
            if let syn::Item::Const(constant) = item {
                objects.push(ObjectParser.parse(constant.clone(), config)?);
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
    use ligen::parser::assert::*;

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

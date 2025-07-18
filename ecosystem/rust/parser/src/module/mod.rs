//! Module representation.

mod import;
pub use import::*;

use syn::spanned::Spanned;
use ligen::ir::{Function, Module, Import, TypeDefinition, Interface, Object};
use crate::prelude::*;
use crate::{RustInterfaceParser, RustTypeAliasParser, RustObjectParser, RustFunctionParser, RustIdentifierParser, RustAttributesParser, RustEnumerationParser, RustStructureParser, RustVisibilityParser};

#[derive(Default)]
pub struct RustModuleParser {
    interface_parser: RustInterfaceParser,
    object_parser: RustObjectParser,
    visibility_parser: RustVisibilityParser,
    function_parser: RustFunctionParser,
    identifier_parser: RustIdentifierParser,
    attributes_parser: RustAttributesParser,
    type_alias_parser: RustTypeAliasParser,
    enumeration_parser: RustEnumerationParser,
    structure_parser: RustStructureParser,
    imports_parser: RustImportsParser,
}

impl RustModuleParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Parser<Module> for RustModuleParser {
    fn parse(&self, input: impl AsRef<str>, config: &Config) -> Result<Module> {
        syn::parse_str::<syn::ItemMod>(input.as_ref())
            .map_err(|e| Error::Message(format!("Failed to parse module: {e:?}")))
            .and_then(|module| self.transform(module, config))
    }
}

impl Transformer<proc_macro::TokenStream, Module> for RustModuleParser {
    fn transform(&self, input: proc_macro::TokenStream, config: &Config) -> Result<Module> {
        self.transform(proc_macro2::TokenStream::from(input), config)
    }
}

impl Transformer<proc_macro2::TokenStream, Module> for RustModuleParser {
    fn transform(&self, token_stream: proc_macro2::TokenStream, config: &Config) -> Result<Module> {
        syn::parse2::<syn::ItemMod>(token_stream)
            .map_err(|e| Error::Message(format!("Failed to parse module: {e:?}")))
            .and_then(|module| self.transform(module, config))
    }
}

impl Transformer<syn::ItemMod, Module> for RustModuleParser {
    fn transform(&self, module: syn::ItemMod, config: &Config) -> Result<Module> {
        let items = module
            .content
            .map(|(_, items)| items)
            .ok_or("Module file isn't loaded.")?;
        let attributes = self.attributes_parser.transform(module.attrs, config)?;
        let visibility = self.visibility_parser.transform(module.vis, config)?;
        let identifier = self.identifier_parser.transform(module.ident, config)?;

        let imports = self.extract_imports(items.as_slice(), config)?;
        let functions = self.extract_functions(items.as_slice(), config)?;
        let objects = self.extract_objects(items.as_slice(), config)?;
        let types = self.extract_types(items.as_slice(), config)?;
        let interfaces = self.extract_interfaces(items.as_slice())?;
        let modules = self.extract_modules(items, config)?;
        Ok(Module { attributes, visibility, identifier, imports, functions, objects, types, interfaces, modules })
    }
}

impl Transformer<&std::path::Path, Module> for RustModuleParser {
    fn transform(&self, path: &std::path::Path, config: &Config) -> Result<Module> {
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
        self.transform(module, config)
    }
}

impl RustModuleParser {
    fn extract_interfaces(&self, items: &[syn::Item]) -> Result<Vec<Interface>> {
        let mut interfaces = Vec::new();
        for item in items {
            if let syn::Item::Impl(impl_) = item {
                if let Ok(interface) = self.interface_parser.transform(impl_.clone(), &Config::default()) {
                    interfaces.push(interface);
                }
            }
        }
        Ok(interfaces)
    }
    fn extract_types(&self, items: &[syn::Item], config: &Config) -> Result<Vec<TypeDefinition>> {
        let mut types = Vec::new();
        for item in items {
            match item {
                syn::Item::Enum(enumeration) =>
                    types.push(self.enumeration_parser.transform(enumeration.clone(), config)?),
                syn::Item::Struct(structure) =>
                    types.push(self.structure_parser.transform(structure.clone(), config)?),
                syn::Item::Type(type_) => {
                    types.push(self.type_alias_parser.transform(type_.clone(), config)?);
                },
                syn::Item::Union(_union) => {
                    todo!("Union object isn't implemented yet.")
                },
                _ => ()
            }
        }
        Ok(types)
    }

    fn extract_imports(&self, items: &[syn::Item], config: &Config) -> Result<Vec<Import>> {
        let mut imports: Vec<Import> = Default::default();
        for item in items {
            if let syn::Item::Use(import) = item {
                imports.append(&mut self.imports_parser.transform(import.clone(), config)?);
            }
        }
        Ok(imports)
    }
    fn extract_functions(&self, items: &[syn::Item], config: &Config) -> Result<Vec<Function>> {
        let mut functions = Vec::new();
        for item in items {
            if let syn::Item::Fn(function) = item {
                functions.push(self.function_parser.transform(function.clone(), config)?);
            }
        }
        Ok(functions)
    }

    fn extract_modules(&self, items: Vec<syn::Item>, config: &Config) -> Result<Vec<Module>> {
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
            modules.push(self.transform(module, config)?)
        }
        Ok(modules)
    }

    fn extract_objects(&self, items: &[syn::Item], config: &Config) -> Result<Vec<Object>> {
        let mut objects = Vec::new();
        for item in items {
            if let syn::Item::Const(constant) = item {
                objects.push(self.object_parser.transform(constant.clone(), config)?);
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
    use ligen::transformer::assert::*;

    #[test]
    fn module_file() -> Result<()> {
        assert_failure(RustModuleParser::default(), "mod module;")
    }

    #[test]
    fn sub_modules() -> Result<()> {
        assert_eq(RustModuleParser::default(), mock::sub_modules(), quote! {
            pub mod root {
                pub mod branch {
                    pub mod leaf {}
                }
            }
        })
    }

    #[test]
    fn module_types() -> Result<()> {
        assert_eq(RustModuleParser::default(), mock::module_types(), quote! {
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

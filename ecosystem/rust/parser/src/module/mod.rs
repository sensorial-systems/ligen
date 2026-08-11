//! Module representation.

mod import;
pub use import::*;

use crate::prelude::*;
use crate::{
    RustAttributesParser, RustEnumerationParser, RustFunctionParser, RustIdentifierParser,
    RustInterfaceParser, RustObjectParser, RustStructureParser, RustTypeAliasParser,
    RustVisibilityParser,
};
use ligen::idl::{Function, Import, Interface, Module, Object, TypeDefinition};
use syn::spanned::Spanned;

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
    literal_parser: crate::literal::RustLiteralParser,
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

        // Everything below is parsed against a config carrying this module's constants, so that a
        // constant, an array length or a nested module can refer to one by name.
        let config = &self.constants(items.as_slice(), config);
        let imports = self.extract_imports(items.as_slice(), config)?;
        let functions = self.extract_functions(items.as_slice(), config)?;
        let objects = self.extract_objects(items.as_slice(), config)?;
        let types = self.extract_types(items.as_slice(), config)?;
        let interfaces = self.extract_interfaces(items.as_slice())?;
        let modules = self.extract_modules(items, config)?;
        Ok(Module {
            attributes,
            visibility,
            identifier,
            imports,
            functions,
            objects,
            types,
            interfaces,
            modules,
        })
    }
}

impl Transformer<&std::path::Path, Module> for RustModuleParser {
    fn transform(&self, path: &std::path::Path, config: &Config) -> Result<Module> {
        let module = syn2::file_parser::parse_file_recursive(path)?;
        let ident = syn::Ident::new(
            path.file_stem()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default(),
            module.span(),
        ); // FIXME: This is hardcoded.
        let attrs = module.attrs;
        let pub_token = Default::default();
        let semi = Default::default();
        let mod_token = Default::default();
        let content = Some((Default::default(), module.items));
        let vis = syn::Visibility::Public(pub_token);
        let unsafety = Default::default();
        let module = syn::ItemMod {
            unsafety,
            attrs,
            vis,
            mod_token,
            ident,
            semi,
            content,
        };
        self.transform(module, config)
    }
}

impl RustModuleParser {
    fn extract_interfaces(&self, items: &[syn::Item]) -> Result<Vec<Interface>> {
        let mut interfaces = Vec::new();
        for item in items {
            if let syn::Item::Impl(impl_) = item {
                if let Ok(interface) = self
                    .interface_parser
                    .transform(impl_.clone(), &Config::default())
                {
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
                syn::Item::Enum(enumeration) => types.push(
                    self.enumeration_parser
                        .transform(enumeration.clone(), config)?,
                ),
                syn::Item::Struct(structure) => {
                    types.push(self.structure_parser.transform(structure.clone(), config)?)
                }
                syn::Item::Type(type_) => {
                    types.push(self.type_alias_parser.transform(type_.clone(), config)?);
                }
                syn::Item::Union(_union) => {
                    return Err(Error::Message(
                        "Union object isn't implemented yet.".to_string(),
                    ))
                }
                _ => (),
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
        let items = items.into_iter().filter_map(|item| {
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

    /// The config this module's items are parsed against: the one handed down, plus every constant
    /// declared here that can be worked out.
    ///
    /// Repeated until nothing new resolves, rather than in one pass, because a constant may be
    /// written in terms of one declared below it — Rust does not care what order they appear in and
    /// neither should this. Each round resolves at least one more or stops, so a chain of any depth
    /// settles and a circular one gives up instead of spinning.
    ///
    /// What cannot be worked out is simply left out. That is not a failure: a constant whose value
    /// needs a compiler is still a constant, and the only thing lost is the ability to *refer* to it
    /// from somewhere a literal is required — where it will be reported, in its own right, by the
    /// parser that needed it.
    ///
    /// Inherited, so a nested module sees the enclosing one's constants. Rust would want a `use`
    /// for that; a parser with no import graph would otherwise see nothing at all, and a name that
    /// resolves to the value it has in the file is better than a name that resolves to nothing.
    fn constants(&self, items: &[syn::Item], config: &Config) -> Config {
        let mut config = config.clone();
        let mut pending: Vec<&syn::ItemConst> = items
            .iter()
            .filter_map(|item| match item {
                syn::Item::Const(constant) => Some(constant),
                _ => None,
            })
            .collect();

        while !pending.is_empty() {
            let round = pending.len();
            let mut unresolved = Vec::with_capacity(round);
            for constant in pending {
                match self
                    .literal_parser
                    .transform((*constant.expr).clone(), &config)
                {
                    Ok(value) => {
                        crate::literal::declare_constant(
                            &mut config,
                            &constant.ident.to_string(),
                            value,
                        );
                    }
                    Err(_) => unresolved.push(constant),
                }
            }
            // A round that resolved nothing will resolve nothing next time either.
            if unresolved.len() == round {
                break;
            }
            pending = unresolved;
        }
        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ligen::idl::module::mock;
    use ligen::transformer::assert::*;
    use quote::quote;

    #[test]
    fn module_file() -> Result<()> {
        assert_failure(RustModuleParser::default(), "mod module;")
    }

    #[test]
    fn sub_modules() -> Result<()> {
        assert_eq(
            RustModuleParser::default(),
            mock::sub_modules(),
            quote! {
                pub mod root {
                    pub mod branch {
                        pub mod leaf {}
                    }
                }
            },
        )
    }

    #[test]
    fn module_types() -> Result<()> {
        assert_eq(
            RustModuleParser::default(),
            mock::module_types(),
            quote! {
                pub mod types {
                    pub struct Structure;
                    pub enum Enumeration {}
                }
            },
        )
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

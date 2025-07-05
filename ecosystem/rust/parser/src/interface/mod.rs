use crate::function::{RustFunctionParser, MethodParser};
use crate::macro_attributes::attributes::AttributesParser;
use crate::object::ObjectParser;
use crate::prelude::*;
use crate::types::TypeParser;

use ligen::ir::{Path, Interface, Visibility, Function, Method, Object};


#[derive(Default)]
pub struct RustInterfaceParser {
    type_parser: TypeParser,
    function_parser: RustFunctionParser,
    method_parser: MethodParser,
    object_parser: ObjectParser,
    attributes_parser: AttributesParser,
}

impl RustInterfaceParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Transformer<syn::ItemImpl, Interface> for RustInterfaceParser {
    fn transform(&self, input: syn::ItemImpl, config: &Config) -> Result<Interface> {
        let attributes = self.attributes_parser.transform(input.attrs, config)?;
        let visibility = Visibility::Public;

        // TODO: What should we do with the self type?
        let type_ = self.type_parser.transform(*input.self_ty, config)?;
        let identifier = type_.path.last().clone().into(); // TODO: Fix this

        let functions = self.extract_functions(input.items.as_slice(), config)?;
        let methods = self.extract_methods(input.items.as_slice(), config)?;
        let objects = self.extract_objects(input.items.as_slice(), config)?;
        let interfaces = self.extract_interfaces(input.items.as_slice(), config)?;
        Ok(Interface { attributes, visibility, identifier, methods, objects, functions, interfaces })
    }
}

impl RustInterfaceParser {
    fn extract_interfaces(&self, _items: &[syn::ImplItem], _config: &Config) -> Result<Vec<Path>> {
        Ok(Default::default())
    }

    fn extract_methods(&self, items: &[syn::ImplItem], config: &Config) -> Result<Vec<Method>> {
        let mut methods = Vec::new();
        for item in items {
            if let syn::ImplItem::Fn(method) = item {
                if let Ok(method) = self.method_parser.transform(method.clone(), config) {
                    methods.push(method);
                }
            }
        }
        Ok(methods)
    }

    fn extract_objects(&self, items: &[syn::ImplItem], config: &Config) -> Result<Vec<Object>> {
        let mut objects = Vec::new();
        for item in items {
            if let syn::ImplItem::Const(object) = item {
                objects.push(self.object_parser.transform(object.clone(), config)?);
            }
        }
        Ok(objects)
    }

    fn extract_functions(&self, items: &[syn::ImplItem], config: &Config) -> Result<Vec<Function>> {
        let mut functions = Vec::new();
        for item in items {
            if let syn::ImplItem::Fn(function) = item {
                if let Ok(function) = self.function_parser.transform(function.clone(), config) {
                    functions.push(function);
                }
            }
        }
        Ok(functions)
    }
}
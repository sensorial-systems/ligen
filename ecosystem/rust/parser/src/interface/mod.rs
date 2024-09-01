use crate::function::{FunctionParser, MethodParser};
use crate::macro_attributes::attributes::AttributesParser;
use crate::object::ObjectParser;
use crate::prelude::*;
use crate::types::TypeParser;

use ligen::parser::{Parser, ParserConfig};
use ligen::ir::{Path, Interface, Visibility, Function, Method, Object};


#[derive(Default)]
pub struct InterfaceParser {}

impl InterfaceParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Parser<syn::ItemImpl> for InterfaceParser {
    type Output = Interface;
    fn parse(&self, input: syn::ItemImpl, config: &ParserConfig) -> Result<Self::Output> {
        let attributes = AttributesParser::default().parse(input.attrs, config)?;
        let visibility = Visibility::Public;

        // TODO: What should we do with the self type?
        let type_ = TypeParser::new().parse(*input.self_ty, config)?;
        let identifier = type_.path.last().clone().into(); // TODO: Fix this

        let functions = self.extract_functions(input.items.as_slice(), config)?;
        let methods = self.extract_methods(input.items.as_slice(), config)?;
        let objects = self.extract_objects(input.items.as_slice(), config)?;
        let interfaces = self.extract_interfaces(input.items.as_slice(), config)?;
        Ok(Interface { attributes, visibility, identifier, methods, objects, functions, interfaces })
    }
}

impl InterfaceParser {
    fn extract_interfaces(&self, _items: &[syn::ImplItem], _config: &ParserConfig) -> Result<Vec<Path>> {
        Ok(Default::default())
    }

    fn extract_methods(&self, items: &[syn::ImplItem], config: &ParserConfig) -> Result<Vec<Method>> {
        let mut methods = Vec::new();
        for item in items {
            if let syn::ImplItem::Fn(method) = item {
                if let Ok(method) = MethodParser::new().parse(method.clone(), config) {
                    methods.push(method);
                }
            }
        }
        Ok(methods)
    }

    fn extract_objects(&self, items: &[syn::ImplItem], config: &ParserConfig) -> Result<Vec<Object>> {
        let mut objects = Vec::new();
        for item in items {
            if let syn::ImplItem::Const(object) = item {
                objects.push(ObjectParser::new().parse(object.clone(), config)?);
            }
        }
        Ok(objects)
    }

    fn extract_functions(&self, items: &[syn::ImplItem], config: &ParserConfig) -> Result<Vec<Function>> {
        let mut functions = Vec::new();
        for item in items {
            if let syn::ImplItem::Fn(function) = item {
                if let Ok(function) = FunctionParser::new().parse(function.clone(), config) {
                    functions.push(function);
                }
            }
        }
        Ok(functions)
    }
}
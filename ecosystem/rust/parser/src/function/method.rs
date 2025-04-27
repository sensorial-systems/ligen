use ligen::ir::Mutability;
use crate::prelude::*;

use ligen::ir::{Attributes, Method, Parameter, Type};
use ligen::parser::prelude::*;
use crate::function::parameter::ParameterParser;
use crate::function::SynchronyParser;
use crate::identifier::IdentifierParser;
use crate::macro_attributes::attributes::AttributeParser;
use crate::types::TypeParser;
use crate::visibility::VisibilityParser;

#[derive(Default)]
pub struct MethodParser {
    identifier_parser: IdentifierParser,
    visibility_parser: VisibilityParser,
    synchrony_parser: SynchronyParser,
    parameter_parser: ParameterParser,
    type_parser: TypeParser,
    attribute_parser: AttributeParser,
}

impl MethodParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Transformer<syn::ImplItemFn, Method> for MethodParser {
    fn transform(&self, method: syn::ImplItemFn, config: &Config) -> Result<Method> {
        if let Some(receiver) = method.sig.receiver() {
            let mutability = if receiver.mutability.is_some() { Mutability::Mutable } else { Mutability::Constant };
            let syn::Signature { asyncness, ident, inputs, output, .. } = method.sig;
            let inputs: Vec<Parameter> = inputs
                .clone()
                .into_iter()
                .filter(|input| !matches!(input, syn::FnArg::Receiver(_)))
                .map(|x| self.parameter_parser.transform(x, config).expect("Failed to convert Parameter"))
                .collect();
            let output: Option<Type> = match output {
                syn::ReturnType::Default => None,
                syn::ReturnType::Type(_x, y) => {
                    Some(self.type_parser.transform(*y, config)?)
                }
            };
            Ok(Method {
                mutability,
                attributes: Attributes {
                    attributes: method
                        .attrs
                        .into_iter()
                        .map(|attribute| self.attribute_parser.transform(attribute, config).expect("Failed to parse meta."))
                        .collect(),
                },
                visibility: self.visibility_parser.transform(method.vis, config)?,
                synchrony: self.synchrony_parser.transform(asyncness, config)?,
                identifier: self.identifier_parser.transform(ident, config)?,
                inputs,
                output,
            })
        } else {
            Err(Error::Message("Function is not a method.".to_string()))
        }
    }
}
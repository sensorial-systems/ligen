use ligen::ir::Mutability;
use crate::prelude::*;

use ligen::ir::{Attributes, Method, Parameter, Type};
use ligen::parsing::parser::{Parser, ParserConfig};
use crate::function::parameter::ParameterParser;
use crate::function::SynchronyParser;
use crate::identifier::IdentifierParser;
use crate::macro_attributes::attributes::AttributeParser;
use crate::types::TypeParser;
use crate::visibility::VisibilityParser;

pub struct MethodParser;

impl Parser<syn::ImplItemMethod> for MethodParser {
    type Output = Method;
    fn parse(&self, method: syn::ImplItemMethod, config: &ParserConfig) -> Result<Self::Output> {
        let mutability = method.sig.receiver().map(|arg| {
            match arg {
                syn::FnArg::Receiver(receiver) => if receiver.mutability.is_some() { Mutability::Mutable } else { Mutability::Constant },
                syn::FnArg::Typed(_pat) => Mutability::Constant // FIXME: This needs better treatment.
            }
        }).unwrap_or(Mutability::Constant);
        let syn::Signature {
            asyncness,
            ident,
            inputs,
            output,
            ..
        } = method.sig;
        let inputs: Vec<Parameter> = inputs
            .clone()
            .into_iter()
            .filter(|input| matches!(input, syn::FnArg::Receiver(_)))
            .map(|x| ParameterParser.parse(x, config).expect("Failed to convert Parameter"))
            .collect();
        let output: Option<Type> = match output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_x, y) => {
                Some(TypeParser.parse(*y, config)?)
            }
        };
        Ok(Self::Output {
            mutability,
            attributes: Attributes {
                attributes: method
                    .attrs
                    .into_iter()
                    .map(|attribute| AttributeParser::default().parse(attribute, config).expect("Failed to parse meta."))
                    .collect(),
            },
            visibility: VisibilityParser.parse(method.vis, config)?,
            synchrony: SynchronyParser.parse(asyncness, config)?,
            identifier: IdentifierParser::new().parse(ident, config)?,
            inputs,
            output,
        })
    }
}
use ligen_ir::{Mutability, Path};
use crate::prelude::*;

use ligen_ir::{Attributes, Method, Parameter, Type};
use ligen_parsing::Parser;
use crate::function::parameter::ParameterParser;
use crate::function::SynchronyParser;
use crate::identifier::IdentifierParser;
use crate::macro_attributes::attributes::AttributeParser;
use crate::types::TypeParser;
use crate::visibility::VisibilityParser;

pub struct MethodParser;

impl Parser<syn::ImplItemMethod> for MethodParser {
    type Output = Method;
    fn parse(&self, method: syn::ImplItemMethod) -> Result<Self::Output> {
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
            .filter(|input| if let syn::FnArg::Receiver(_) = input { false } else { true })
            .map(|x| ParameterParser.parse(x).expect("Failed to convert Parameter"))
            .collect();
        let output: Option<Type> = match output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_x, y) => {
                Some(TypeParser.parse(*y)?)
            }
        };
        // FIXME: Hardcoded.
        let path = Path::default();
        let owner = Type::Composite(path, Default::default());
        Ok(Self::Output {
            owner,
            mutability,
            attributes: Attributes {
                attributes: method
                    .attrs
                    .into_iter()
                    .map(|attribute| AttributeParser.parse(attribute).expect("Failed to parse meta."))
                    .collect(),
            },
            visibility: VisibilityParser.parse(method.vis)?,
            synchrony: SynchronyParser.parse(asyncness)?,
            identifier: IdentifierParser.parse(ident)?,
            inputs,
            output,
        })
    }
}
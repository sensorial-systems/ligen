use syn::FnArg;
use ligen_ir::{Function, Identifier, Mutability, Path};
use crate::prelude::*;

use crate::{Synchrony, Attributes, Method, Parameter, Type, Visibility};

impl From<SynImplItemMethod> for Method {
    fn from(SynImplItemMethod(method): SynImplItemMethod) -> Self {
        let mutability = method.sig.receiver().map(|arg| {
            match arg {
                FnArg::Receiver(receiver) => if receiver.mutability.is_some() { Mutability::Mutable } else { Mutability::Constant },
                FnArg::Typed(_pat) => Mutability::Constant // FIXME: This needs better treatment.
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
            .filter(|input| if let FnArg::Receiver(_) = input { false } else { true })
            .map(|x| SynFnArg::from(x).try_into().expect("Failed to convert Parameter"))
            .collect();
        let output: Option<Type> = match output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_x, y) => {
                Some(Type::try_from(SynType::from(*y)).expect("Failed to convert from ReturnType::Type"))
            }
        };
        // FIXME: Hardcoded.
        let path = Path::default();
        let owner = Type::Composite(path, Default::default());
        Self {
            owner,
            mutability,
            attributes: Attributes {
                attributes: method
                    .attrs
                    .into_iter()
                    .map(|x| SynMeta::from(x.parse_meta().expect("Failed to parse Meta")).into())
                    .collect(),
            },
            visibility: Visibility::from(SynVisibility::from(method.vis)),
            synchrony: Synchrony::from(asyncness),
            path: Identifier::from(SynIdent::from(ident)).into(),
            inputs,
            output,
        }
    }
}

// FIXME: Can we make this a subset of method? Use Method::from and then just catch the things we care about.
impl From<SynImplItemMethod> for Function {
    fn from(SynImplItemMethod(method): SynImplItemMethod) -> Self {
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
            .map(|x| SynFnArg::from(x).try_into().expect("Failed to convert Parameter"))
            .collect();
        let output: Option<Type> = match output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_x, y) => {
                Some(Type::try_from(SynType::from(*y)).expect("Failed to convert from ReturnType::Type"))
            }
        };
        Self {
            attributes: Attributes {
                attributes: method
                    .attrs
                    .into_iter()
                    .map(|x| SynMeta::from(x.parse_meta().expect("Failed to parse Meta")).into())
                    .collect(),
            },
            visibility: Visibility::from(SynVisibility::from(method.vis)),
            synchrony: match asyncness {
                Some(_x) => Some(Async),
                None => None,
            },
            path: Identifier::from(SynIdent::from(ident)).into(),
            inputs,
            output,
        }
    }
}

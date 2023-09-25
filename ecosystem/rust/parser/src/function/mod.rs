use crate::prelude::*;

use ligen_ir::{Attributes, Function, Parameter, Type};
use ligen_parsing::Parser;
use crate::function::parameter::ParameterParser;
use crate::identifier::IdentifierParser;
use crate::macro_attributes::attributes::AttributeParser;
use crate::types::TypeParser;

mod parameter;
mod method;
mod synchrony;

pub use parameter::*;
pub use method::*;
pub use synchrony::*;
use crate::visibility::VisibilityParser;

#[derive(Default)]
pub struct FunctionParser;

impl Parser<syn::ItemFn> for FunctionParser {
    type Output = Function;
    fn parse(&self, item_fn: syn::ItemFn) -> Result<Self::Output> {
        let syn::Signature {
            asyncness,
            ident,
            inputs,
            output,
            ..
        } = item_fn.sig;
        let inputs: Vec<Parameter> = inputs
            .clone()
            .into_iter()
            .map(|x| ParameterParser.parse(x).expect("Failed to convert Parameter"))
            .collect();
        let output: Option<Type> = match output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_x, y) => {
                Some(TypeParser.parse(*y)?)
            }
        };
        Ok(Self::Output {
            attributes: Attributes {
                attributes: item_fn
                    .attrs
                    .into_iter()
                    .map(|attribute| AttributeParser.parse(attribute).expect("Failed to parse Meta"))
                    .collect(),
            },
            visibility: VisibilityParser.parse(item_fn.vis)?,
            synchrony: SynchronyParser.parse(asyncness)?,
            identifier: IdentifierParser.parse(ident)?,
            inputs,
            output,
        })
    }
}

// FIXME: Can we make this a subset of method? What about using the MethodParser and then just catch the things we care about.
//  This is repeating Parser<syn::ImplItemMethod> for MethodParser.
impl Parser<syn::ImplItemMethod> for FunctionParser {
    type Output = Function;
    fn parse(&self, method: syn::ImplItemMethod) -> Result<Self::Output> {
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
            .map(|x| ParameterParser.parse(x).expect("Failed to convert Parameter"))
            .collect();
        let output: Option<Type> = match output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_x, y) => {
                Some(TypeParser.parse(*y).expect("Failed to convert from ReturnType::Type"))
            }
        };
        Ok(Self::Output {
            attributes: Attributes {
                attributes: method
                    .attrs
                    .into_iter()
                    .map(|attribute| AttributeParser.parse(attribute).expect("Failed to parse Meta"))
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

impl Parser<proc_macro::TokenStream> for FunctionParser {
    type Output = Function;
    fn parse(&self, token_stream: proc_macro::TokenStream) -> Result<Self::Output> {
        self.parse(proc_macro2::TokenStream::from(token_stream))
    }
}

impl Parser<proc_macro2::TokenStream> for FunctionParser {
    type Output = Function;
    fn parse(&self, token_stream: proc_macro2::TokenStream) -> Result<Self::Output> {
        syn::parse2::<syn::ItemFn>(token_stream)
            .map_err(|e| Error::Message(format!("Failed to parse function: {:?}", e)))
            .and_then(|function| self.parse(function))
    }
}

#[cfg(test)]
mod test {
    use ligen_parsing::assert::assert_eq;
    use crate::function::FunctionParser;
    use crate::prelude::*;

    use ligen_ir::function::mock;

    #[test]
    fn function() -> Result<()> {
        assert_eq(FunctionParser, mock::function(), quote! {
            fn test() {}
        })
    }

    #[test]
    fn function_pub() -> Result<()> {
        assert_eq(FunctionParser, mock::function_pub(), quote! {
            pub fn test() {}
        })
    }

    #[test]
    fn function_input() -> Result<()> {
        assert_eq(FunctionParser, mock::function_input(), quote! {
            fn test(a: String, b: String) {}
        })
    }

    #[test]
    fn function_output() -> Result<()> {
        assert_eq(FunctionParser, mock::function_output(), quote! {
            fn test() -> String {}
        })
    }

    #[test]
    fn function_input_output() -> Result<()> {
        assert_eq(FunctionParser, mock::function_input_output(), quote! {
            fn test(a: String, b: &String, c: &mut String) -> &String {}
        })
    }

    #[test]
    fn function_attribute() -> Result<()> {
        assert_eq(FunctionParser, mock::function_attribute(), quote! {
            #[test(a = "b")]
            fn test() {}
        })
    }

    #[test]
    fn function_async() -> Result<()> {
        assert_eq(FunctionParser, mock::function_async(), quote! {
            async fn test() {}
        })
    }

    #[test]
    fn function_complete() -> Result<()> {
        assert_eq(FunctionParser, mock::function_complete(), quote! {
            #[test(a = "b")]
            async fn test(a: String, b: &String, c: &mut String) -> &String {}
        })
    }
}

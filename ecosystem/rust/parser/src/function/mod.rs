use crate::prelude::*;

use ligen::ir::{Function, Parameter, Type};
use ligen::parser::{Parser, ParserConfig};
use crate::function::parameter::ParameterParser;
use crate::identifier::IdentifierParser;
use crate::macro_attributes::attributes::AttributesParser;
use crate::types::TypeParser;

mod parameter;
mod method;
mod synchrony;

pub use method::*;
pub use synchrony::*;
use crate::visibility::VisibilityParser;

#[derive(Default)]
pub struct FunctionParser;

impl FunctionParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Parser<syn::ItemFn> for FunctionParser {
    type Output = Function;
    fn parse(&self, item_fn: syn::ItemFn, config: &ParserConfig) -> Result<Self::Output> {
        let attributes = AttributesParser::default().parse(item_fn.attrs, config)?;
        let visibility = VisibilityParser.parse(item_fn.vis, config)?;
        let synchrony = SynchronyParser.parse(item_fn.sig.asyncness, config)?;
        let identifier = IdentifierParser::new().parse(item_fn.sig.ident, config)?;
        let inputs = self.parse_inputs(item_fn.sig.inputs, config)?;
        let output = self.parse_output(item_fn.sig.output, config)?;
        Ok(Self::Output { attributes, visibility, synchrony, identifier, inputs, output })
    }
}

impl Parser<syn::ImplItemFn> for FunctionParser {
    type Output = Function;
    fn parse(&self, function: syn::ImplItemFn, config: &ParserConfig) -> Result<Self::Output> {
        if function.sig.receiver().is_some() {
            Err(Error::Message("Function is not a method.".to_string()))
        } else {
            let attributes = AttributesParser::default().parse(function.attrs, config)?;
            let visibility = VisibilityParser.parse(function.vis, config)?;
            let synchrony = SynchronyParser.parse(function.sig.asyncness, config)?;
            let identifier = IdentifierParser::new().parse(function.sig.ident, config)?;
            let inputs = self.parse_inputs(function.sig.inputs, config)?;
            let output = self.parse_output(function.sig.output, config)?;
            Ok(Self::Output { attributes, visibility, synchrony, identifier, inputs, output })    
        }
    }
}

impl FunctionParser {
    fn parse_output(&self, output: syn::ReturnType, config: &ParserConfig) -> Result<Option<Type>> {
        Ok(match output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_x, y) => {
                Some(TypeParser::new().parse(*y, config)?)
            }
        })
    }
    fn parse_inputs(&self, args: syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>, config: &ParserConfig) -> Result<Vec<Parameter>> {
        let mut parameters = Vec::new();
        for arg in args {
            parameters.push(ParameterParser.parse(arg, config)?);
        }
        Ok(parameters)
    }
}

impl Parser<proc_macro::TokenStream> for FunctionParser {
    type Output = Function;
    fn parse(&self, token_stream: proc_macro::TokenStream, config: &ParserConfig) -> Result<Self::Output> {
        self.parse(proc_macro2::TokenStream::from(token_stream), config)
    }
}

impl Parser<proc_macro2::TokenStream> for FunctionParser {
    type Output = Function;
    fn parse(&self, token_stream: proc_macro2::TokenStream, config: &ParserConfig) -> Result<Self::Output> {
        syn::parse2::<syn::ItemFn>(token_stream)
            .map_err(|e| Error::Message(format!("Failed to parse function: {:?}", e)))
            .and_then(|function| self.parse(function, config))
    }
}

impl Parser<&str> for FunctionParser {
    type Output = Function;
    fn parse(&self, input: &str, config: &ParserConfig) -> Result<Self::Output> {
        syn::parse_str::<syn::ItemFn>(input)
            .map_err(|e| Error::Message(format!("Failed to parse function: {:?}", e)))
            .and_then(|function| self.parse(function, config))
    }
}

#[cfg(test)]
mod test {
    use ligen::parser::assert::assert_eq;
    use crate::function::FunctionParser;
    use crate::prelude::*;

    use ligen::ir::function::mock;

    #[test]
    fn function() -> Result<()> {
        assert_eq(FunctionParser, mock::function(), "pub fn test() {}")
    }

    #[test]
    fn function_input() -> Result<()> {
        assert_eq(FunctionParser, mock::function_input(), "pub fn test(a: i32, b: i32) {}")
    }

    #[test]
    fn function_output() -> Result<()> {
        assert_eq(FunctionParser, mock::function_output(), "pub fn test() -> String {}")
    }

    #[test]
    fn function_input_output() -> Result<()> {
        assert_eq(FunctionParser, mock::function_input_output(), "pub fn test(a: i32, b: i32) -> i32 {}")
    }

    #[test]
    fn function_attribute() -> Result<()> {
        assert_eq(FunctionParser, mock::function_attribute(), "#[test(a = \"b\")] pub fn test() {}")
    }

    #[test]
    fn function_async() -> Result<()> {
        assert_eq(FunctionParser, mock::function_async(), "pub async fn test() {}")
    }

    #[test]
    fn function_complete() -> Result<()> {
        assert_eq(FunctionParser, mock::function_complete(), "#[test(a = \"b\")] pub async fn test(a: String, b: &String, c: &mut String) -> &String {}")
    }
}

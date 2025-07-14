use crate::prelude::*;

use ligen::ir::{Function, Parameter, Type};
use crate::function::parameter::ParameterParser;
use crate::identifier::IdentifierParser;
use crate::block::BlockParser;
use crate::macro_attributes::attributes::AttributesParser;
use crate::types::TypeParser;

mod parameter;
mod method;
mod synchrony;

pub use method::*;
pub use synchrony::*;
use crate::visibility::VisibilityParser;

#[derive(Default)]
pub struct RustFunctionParser {
    identifier_parser: IdentifierParser,
    attributes_parser: AttributesParser,
    visibility_parser: VisibilityParser,
    synchrony_parser: SynchronyParser,
    parameter_parser: ParameterParser,
    type_parser: TypeParser,
    block_parser: BlockParser
}

impl RustFunctionParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Transformer<syn::ItemFn, Function> for RustFunctionParser {
    fn transform(&self, item_fn: syn::ItemFn, config: &Config) -> Result<Function> {
        let attributes = self.attributes_parser.transform(item_fn.attrs, config)?;
        let visibility = self.visibility_parser.transform(item_fn.vis, config)?;
        let synchrony = self.synchrony_parser.transform(item_fn.sig.asyncness, config)?;
        let identifier = self.identifier_parser.transform(item_fn.sig.ident, config)?;
        let inputs = self.parse_inputs(item_fn.sig.inputs, config)?;
        let output = self.parse_output(item_fn.sig.output, config)?;
        let body = self.block_parser.transform(item_fn.block, config)?;
        Ok(Function { attributes, visibility, synchrony, identifier, inputs, output, body })
    }
}

impl Transformer<syn::ImplItemFn, Function> for RustFunctionParser {
    fn transform(&self, function: syn::ImplItemFn, config: &Config) -> Result<Function> {
        if function.sig.receiver().is_some() {
            Err(Error::Message("Function is not a method.".to_string()))
        } else {
            let attributes = self.attributes_parser.transform(function.attrs, config)?;
            let visibility = self.visibility_parser.transform(function.vis, config)?;
            let synchrony = self.synchrony_parser.transform(function.sig.asyncness, config)?;
            let identifier = self.identifier_parser.transform(function.sig.ident, config)?;
            let inputs = self.parse_inputs(function.sig.inputs, config)?;
            let output = self.parse_output(function.sig.output, config)?;
            let body = self.block_parser.transform(function.block, config)?;
            Ok(Function { attributes, visibility, synchrony, identifier, inputs, output, body })
        }
    }
}

impl RustFunctionParser {
    fn parse_output(&self, output: syn::ReturnType, config: &Config) -> Result<Option<Type>> {
        Ok(match output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_x, y) => {
                Some(self.type_parser.transform(*y, config)?)
            }
        })
    }
    fn parse_inputs(&self, args: syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>, config: &Config) -> Result<Vec<Parameter>> {
        let mut parameters = Vec::new();
        for arg in args {
            parameters.push(self.parameter_parser.transform(arg, config)?);
        }
        Ok(parameters)
    }
}

impl Transformer<proc_macro::TokenStream, Function> for RustFunctionParser {
    fn transform(&self, token_stream: proc_macro::TokenStream, config: &Config) -> Result<Function> {
        self.transform(proc_macro2::TokenStream::from(token_stream), config)
    }
}

impl Transformer<proc_macro2::TokenStream, Function> for RustFunctionParser {
    fn transform(&self, token_stream: proc_macro2::TokenStream, config: &Config) -> Result<Function> {
        syn::parse2::<syn::ItemFn>(token_stream)
            .map_err(|e| Error::Message(format!("Failed to parse function: {e:?}")))
            .and_then(|function| self.transform(function, config))
    }
}

impl Parser<Function> for RustFunctionParser {
    fn parse(&self, input: impl AsRef<str>, config: &Config) -> Result<Function> {
        syn::parse_str::<syn::ItemFn>(input.as_ref())
            .map_err(|e| Error::Message(format!("Failed to parse function: {e:?}")))
            .and_then(|function| self.transform(function, config))
    }
}

#[cfg(test)]
mod test {
    use ligen::transformer::assert::assert_eq;
    use crate::function::RustFunctionParser;
    use crate::prelude::*;

    use ligen::ir::function::mock;

    #[test]
    fn function() -> Result<()> {
        assert_eq(RustFunctionParser::default(), mock::function(), "pub fn test() {}")
    }

    #[test]
    fn function_input() -> Result<()> {
        assert_eq(RustFunctionParser::default(), mock::function_input(), "pub fn test(a: i32, b: i32) {}")
    }

    #[test]
    fn function_output() -> Result<()> {
        assert_eq(RustFunctionParser::default(), mock::function_output(), "pub fn test() -> String {}")
    }

    #[test]
    fn function_input_output() -> Result<()> {
        assert_eq(RustFunctionParser::default(), mock::function_input_output(), "pub fn test(a: i32, b: i32) -> i32 {}")
    }

    #[test]
    fn function_attribute() -> Result<()> {
        assert_eq(RustFunctionParser::default(), mock::function_attribute(), "#[test(a = \"b\")] pub fn test() {}")
    }

    #[test]
    fn function_async() -> Result<()> {
        assert_eq(RustFunctionParser::default(), mock::function_async(), "pub async fn test() {}")
    }

    #[test]
    fn function_complete() -> Result<()> {
        assert_eq(RustFunctionParser::default(), mock::function_complete(), "#[test(a = \"b\")] pub async fn test(a: String, b: &String, c: &mut String) -> &String {}")
    }
}

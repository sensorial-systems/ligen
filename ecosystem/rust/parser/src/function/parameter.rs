//! Function parameter.

use crate::prelude::*;
use ligen::ir::{Identifier, Type, Mutability, Parameter};
use ligen::parser::prelude::*;
use crate::identifier::IdentifierParser;
use crate::macro_attributes::attributes::AttributesParser;
use crate::types::TypeParser;

#[derive(Default)]
pub struct ParameterParser {
    identifier_parser: IdentifierParser,
    type_parser: TypeParser,
    attributes_parser: AttributesParser,
}

impl Transformer<syn::FnArg, Parameter> for ParameterParser {
    fn transform(&self, fn_arg: syn::FnArg, config: &Config) -> Result<Parameter> {
        match fn_arg {
            syn::FnArg::Typed(syn::PatType { pat, ty, attrs, .. }) => {
                if let syn::Pat::Ident(syn::PatIdent { ident, .. }) = *pat {
                    Ok(Parameter {
                        attributes: self.attributes_parser.transform(attrs, config)?,
                        identifier: self.identifier_parser.transform(ident, config)?,
                        type_: self.type_parser.transform(*ty, config)?,
                        default_value: Default::default(),
                    })
                } else {
                    Err(Error::Message("Identifier not found".into()))
                }
            }
            // TODO: Implement translation for syn::Receiver. `Self` should be the fully qualified Type path.
            syn::FnArg::Receiver(syn::Receiver {
                                attrs,
                                reference,
                                mutability,
                                ..
                            }) => {
                let attributes = self.attributes_parser.transform(attrs, config)?;
                let identifier = Identifier::new("self");
                let type_ = reference
                    .map(|_| {
                        let mutability = if mutability.is_none() { Mutability::Constant } else { Mutability::Mutable };
                        let type_ = Type::from(Identifier::new("Self"));
                        Type::reference(mutability, type_)
                    })
                    .unwrap_or_else(|| Type::from(Identifier::new("Self")));
                let default_value = Default::default();
                Ok(Parameter { attributes, identifier, type_, default_value })
            },
        }
    }
}

impl Transformer<proc_macro::TokenStream, Parameter> for ParameterParser {
    fn transform(&self, token_stream: proc_macro::TokenStream, config: &Config) -> Result<Parameter> {
        self.transform(proc_macro2::TokenStream::from(token_stream), config)
    }
}

impl Transformer<proc_macro2::TokenStream, Parameter> for ParameterParser {
    fn transform(&self, input: proc_macro2::TokenStream, config: &Config) -> Result<Parameter> {
        syn::parse2::<syn::FnArg>(input)
            .map_err(|e| Error::Message(format!("Failed to parse parameter: {}", e)))
            .and_then(|parameter| self.transform(parameter, config))
    }
}

impl Parser<Parameter> for ParameterParser {
    fn parse(&self, input: impl AsRef<str>, config: &Config) -> Result<Parameter> {
        syn::parse_str::<syn::FnArg>(input.as_ref())
            .map_err(|e| Error::Message(format!("Failed to parse parameter: {}", e)))
            .and_then(|parameter| self.transform(parameter, config))
    }
}

#[cfg(test)]
mod test {
    use crate::function::parameter::ParameterParser;
    use crate::prelude::*;

    use ligen::ir::function::parameter::mock;
    use ligen::parser::assert::assert_eq;

    #[test]
    fn primitive_parameter() -> Result<()> {
        assert_eq(ParameterParser::default(), mock::primitive_parameter(), "integer: i32")
    }

    #[test]
    fn parameter_attribute() -> Result<()> {
        assert_eq(ParameterParser::default(), mock::parameter_attribute(), "#[attribute] integer: i32")
    }

    #[test]
    fn composite_parameter() -> Result<()> {
        assert_eq(ParameterParser::default(), mock::composite_parameter(), "name: String")
    }

    #[test]
    fn constant_reference_parameter() -> Result<()> {
        assert_eq(ParameterParser::default(), mock::constant_reference_parameter(), "name: &String")
    }

    #[test]
    fn mutable_reference_parameter() -> Result<()> {
        assert_eq(ParameterParser::default(), mock::mutable_reference_parameter(), "name: &mut String")
    }

    #[test]
    fn constant_pointer_parameter() -> Result<()> {
        assert_eq(ParameterParser::default(), mock::constant_reference_parameter(), "name: *const String")
    }

    #[test]
    fn mutable_pointer_parameter() -> Result<()> {
        assert_eq(ParameterParser::default(), mock::mutable_reference_parameter(), "name: *mut String")
    }

    #[test]
    fn receiver_parameter() -> Result<()> {
        assert_eq(ParameterParser::default(), mock::receiver_parameter(), "self")
    }

    #[test]
    fn reference_receiver_parameter() -> Result<()> {
        assert_eq(ParameterParser::default(), mock::reference_receiver_parameter(), "&self")
    }

    #[test]
    fn mutable_receiver_parameter() -> Result<()> {
        assert_eq(ParameterParser::default(), mock::mutable_receiver_parameter(), "&mut self")
    }
}

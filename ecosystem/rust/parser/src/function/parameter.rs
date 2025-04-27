//! Function parameter.

use crate::prelude::*;
use ligen::ir::{Identifier, Type, Mutability, Parameter};
use ligen::parser::prelude::*;
use crate::identifier::IdentifierParser;
use crate::macro_attributes::attributes::AttributesParser;
use crate::types::TypeParser;

pub struct ParameterParser;

impl Parser<syn::FnArg> for ParameterParser {
    type Output = Parameter;

    fn parse(&self, fn_arg: syn::FnArg, config: &Config) -> Result<Self::Output> {
        match fn_arg {
            syn::FnArg::Typed(syn::PatType { pat, ty, attrs, .. }) => {
                if let syn::Pat::Ident(syn::PatIdent { ident, .. }) = *pat {
                    Ok(Self::Output {
                        attributes: AttributesParser::default().parse(attrs, config)?,
                        identifier: IdentifierParser::new().parse(ident, config)?,
                        type_: TypeParser::new().parse(*ty, config)?,
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
                let attributes = AttributesParser::default().parse(attrs, config)?;
                let identifier = Identifier::new("self");
                let type_ = reference
                    .map(|_| {
                        let mutability = if mutability.is_none() { Mutability::Constant } else { Mutability::Mutable };
                        let type_ = Type::from(Identifier::new("Self"));
                        Type::reference(mutability, type_)
                    })
                    .unwrap_or_else(|| Type::from(Identifier::new("Self")));
                let default_value = Default::default();
                Ok(Self::Output { attributes, identifier, type_, default_value })
            },
        }
    }
}

impl Parser<proc_macro::TokenStream> for ParameterParser {
    type Output = Parameter;

    fn parse(&self, token_stream: proc_macro::TokenStream, config: &Config) -> Result<Self::Output> {
        self.parse(proc_macro2::TokenStream::from(token_stream), config)
    }
}

impl Parser<proc_macro2::TokenStream> for ParameterParser {
    type Output = Parameter;

    fn parse(&self, input: proc_macro2::TokenStream, config: &Config) -> Result<Self::Output> {
        syn::parse2::<syn::FnArg>(input)
            .map_err(|e| Error::Message(format!("Failed to parse parameter: {}", e)))
            .and_then(|parameter| self.parse(parameter, config))
    }
}

impl Parser<&str> for ParameterParser {
    type Output = Parameter;

    fn parse(&self, input: &str, config: &Config) -> Result<Self::Output> {
        syn::parse_str::<syn::FnArg>(input)
            .map_err(|e| Error::Message(format!("Failed to parse parameter: {}", e)))
            .and_then(|parameter| self.parse(parameter, config))
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
        assert_eq(ParameterParser, mock::primitive_parameter(), "integer: i32")
    }

    #[test]
    fn parameter_attribute() -> Result<()> {
        assert_eq(ParameterParser, mock::parameter_attribute(), "#[attribute] integer: i32")
    }

    #[test]
    fn composite_parameter() -> Result<()> {
        assert_eq(ParameterParser, mock::composite_parameter(), "name: String")
    }

    #[test]
    fn constant_reference_parameter() -> Result<()> {
        assert_eq(ParameterParser, mock::constant_reference_parameter(), "name: &String")
    }

    #[test]
    fn mutable_reference_parameter() -> Result<()> {
        assert_eq(ParameterParser, mock::mutable_reference_parameter(), "name: &mut String")
    }

    #[test]
    fn constant_pointer_parameter() -> Result<()> {
        assert_eq(ParameterParser, mock::constant_reference_parameter(), "name: *const String")
    }

    #[test]
    fn mutable_pointer_parameter() -> Result<()> {
        assert_eq(ParameterParser, mock::mutable_reference_parameter(), "name: *mut String")
    }

    #[test]
    fn receiver_parameter() -> Result<()> {
        assert_eq(ParameterParser, mock::receiver_parameter(), "self")
    }

    #[test]
    fn reference_receiver_parameter() -> Result<()> {
        assert_eq(ParameterParser, mock::reference_receiver_parameter(), "&self")
    }

    #[test]
    fn mutable_receiver_parameter() -> Result<()> {
        assert_eq(ParameterParser, mock::mutable_receiver_parameter(), "&mut self")
    }
}

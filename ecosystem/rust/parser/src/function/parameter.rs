//! Function parameter.

use crate::prelude::*;
use ligen::ir::{Identifier, Reference, Type, Mutability, Parameter};
use ligen::parsing::parser::Parser;
use crate::identifier::IdentifierParser;
use crate::macro_attributes::attributes::AttributesParser;
use crate::types::TypeParser;

pub struct ParameterParser;

impl Parser<syn::FnArg> for ParameterParser {
    type Output = Parameter;

    fn parse(&self, fn_arg: syn::FnArg) -> Result<Self::Output> {
        match fn_arg {
            syn::FnArg::Typed(syn::PatType { pat, ty, attrs, .. }) => {
                if let syn::Pat::Ident(syn::PatIdent { ident, .. }) = *pat {
                    Ok(Self::Output {
                        attributes: AttributesParser.parse(attrs)?,
                        identifier: IdentifierParser.parse(ident)?,
                        type_: TypeParser.parse(*ty)?,
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
                let attributes = AttributesParser.parse(attrs)?;
                let identifier = Identifier::new("self").into();
                let type_ = reference
                    .map(|_| {
                        let mutability = if mutability.is_none() { Mutability::Constant } else { Mutability::Mutable };
                        let type_ = Box::new(Type::from(Identifier::new("Self")));
                        Type::Reference(Reference { mutability, type_ })
                    })
                    .unwrap_or_else(|| Type::from(Identifier::new("Self")));
                Ok(Self::Output { attributes, identifier, type_ })
            },
        }
    }
}

impl Parser<proc_macro::TokenStream> for ParameterParser {
    type Output = Parameter;

    fn parse(&self, token_stream: proc_macro::TokenStream) -> Result<Self::Output> {
        self.parse(proc_macro2::TokenStream::from(token_stream))
    }
}

impl Parser<proc_macro2::TokenStream> for ParameterParser {
    type Output = Parameter;

    fn parse(&self, input: proc_macro2::TokenStream) -> Result<Self::Output> {
        syn::parse2::<syn::FnArg>(input)
            .map_err(|e| Error::Message(format!("Failed to parse parameter: {}", e)))
            .and_then(|parameter| self.parse(parameter))
    }
}

impl Parser<&str> for ParameterParser {
    type Output = Parameter;

    fn parse(&self, input: &str) -> Result<Self::Output> {
        syn::parse_str::<syn::FnArg>(input)
            .map_err(|e| Error::Message(format!("Failed to parse parameter: {}", e)))
            .and_then(|parameter| self.parse(parameter))
    }
}

impl ToTokens for Parameter {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = self.identifier.to_token_stream();
        let typ = self.type_.to_token_stream();
        tokens.append_all(quote! {#ident: #typ})
    }
}

#[cfg(test)]
mod test {
    use crate::function::parameter::ParameterParser;
    use crate::prelude::*;

    use ligen::ir::function::parameter::mock;
    use ligen::parsing::assert::assert_eq;

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

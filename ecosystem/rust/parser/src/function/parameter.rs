//! Function parameter.

use crate::prelude::*;
use ligen_ir::{Identifier, Reference, Type, Mutability, Parameter};
use ligen_parsing::Parser;
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
            // TODO: Implement conversion for syn::Receiver. <- What does it mean?
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
        let token_stream = proc_macro2::TokenStream::from(token_stream);
        self.parse(token_stream)
    }
}

impl Parser<proc_macro2::TokenStream> for ParameterParser {
    type Output = Parameter;

    fn parse(&self, input: proc_macro2::TokenStream) -> Result<Self::Output> {
        let arg = syn::parse2::<syn::FnArg>(input)
            .map_err(|e| Error::Message(format!("Failed to parse parameter: {}", e)))?;
        self.parse(arg)
    }
}

impl ToTokens for Parameter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = self.identifier.to_token_stream();
        let typ = self.type_.to_token_stream();
        tokens.append_all(quote! {#ident: #typ})
    }
}

#[cfg(test)]
mod test {
    use super::Parameter;
    use ligen_ir::{Identifier, Primitive, Integer, Reference, Type, Attribute, Mutability};
    use ligen_parsing::Parser;
    use crate::function::parameter::ParameterParser;
    use crate::prelude::*;

    #[test]
    fn parameter_primitive() -> Result<()> {
        assert_eq!(
            ParameterParser.parse(quote! { #[attribute] integer: i32 })?,
            Parameter {
                attributes: Attribute::Group("attribute".into(), Default::default()).into(),
                identifier: Identifier::new("integer"),
                type_: Type::Primitive(Primitive::Integer(Integer::I32))
            }
        );
        Ok(())
    }

    #[test]
    fn parameter_composite() -> Result<()> {
        assert_eq!(
            ParameterParser.parse(quote! {name: String})?,
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("name"),
                type_: Type::Composite(Identifier::new("String").into(), Default::default())
            }
        );
        Ok(())
    }

    #[test]
    fn parameter_borrow_constant() -> Result<()> {
        assert_eq!(
            ParameterParser.parse(quote! {name: &String})?,
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("name"),
                type_: Type::Reference(
                    Reference {
                        mutability: Mutability::Constant,
                        type_: Box::new(Type::Composite(Identifier::new("String").into(), Default::default()))
                    }
                )
            }
        );
        Ok(())
    }

    #[test]
    fn parameter_borrow_mutable() -> Result<()> {
        assert_eq!(
            ParameterParser.parse(quote! {name: &mut String})?,
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("name"),
                type_: Type::Reference(
                    Reference {
                        mutability: Mutability::Mutable,
                        type_: Box::new(Type::Composite(Identifier::new("String").into(), Default::default()))
                    }
                )

            }
        );
        Ok(())
    }

    #[test]
    fn parameter_pointer_constant() -> Result<()> {
        assert_eq!(
            ParameterParser.parse(quote! {name: *const String})?,
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("name"),
                type_: Type::Reference(
                    Reference {
                        mutability: Mutability::Constant,
                        type_: Box::new(Type::Composite(Identifier::new("String").into(), Default::default()))
                    }
                )

            }
        );
        Ok(())
    }

    #[test]
    fn parameter_pointer_mutable() -> Result<()> {
        assert_eq!(
            ParameterParser.parse(quote! {name: *mut String})?,
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("name"),
                type_: Type::Reference(
                    Reference {
                        mutability: Mutability::Mutable,
                        type_: Box::new(Type::Composite(Identifier::new("String").into(), Default::default()))
                    }
                )
            }
        );
        Ok(())
    }

    #[test]
    fn receiver_parameter() -> Result<()> {
        assert_eq!(
            ParameterParser.parse(quote! {self})?,
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("self").into(),
                type_: Type::Composite(Identifier::new("Self").into(), Default::default())
            }
        );
        Ok(())
    }

    #[test]
    fn reference_receiver_parameter() -> Result<()> {
        assert_eq!(
            ParameterParser.parse(quote! {&self})?,
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("self").into(),
                type_: Type::Reference(
                    Reference {
                        mutability: Mutability::Constant,
                        type_: Box::new(Type::Composite(Identifier::new("Self").into(), Default::default()))
                    }
                )
            }
        );
        Ok(())
    }

    #[test]
    fn mutable_receiver_parameter() -> Result<()> {
        assert_eq!(
            ParameterParser.parse(quote! {&mut self})?,
            Parameter {
                attributes: Default::default(),
                identifier: Identifier::new("self").into(),
                type_: Type::Reference(
                    Reference {
                        mutability: Mutability::Mutable,
                        type_: Box::new(Type::Composite(Identifier::new("Self").into(), Default::default()))
                    }
                )
            }
        );
        Ok(())
    }
}

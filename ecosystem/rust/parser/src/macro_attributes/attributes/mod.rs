mod attribute;
pub use attribute::*;

use ligen_ir::{Attribute, Attributes, Identifier};
use crate::prelude::*;
use ligen_parsing::Parser;

pub struct AttributesParser;

impl Parser<Vec<syn::Attribute>> for AttributesParser {
    type Output = Attributes;
    fn parse(&self, in_attributes: Vec<syn::Attribute>) -> Result<Self::Output> {
        let mut attributes = Vec::new();
        for attribute in in_attributes {
            attributes.push(AttributeParser.parse(attribute)?);
        }
        Ok(Self::Output { attributes })
    }
}

impl Parser<proc_macro2::TokenStream> for AttributesParser {
    type Output = Attributes;
    fn parse(&self, tokenstream: proc_macro2::TokenStream) -> Result<Self::Output> {
        syn::parse2::<LigenAttributes>(tokenstream)
            .map_err(|e| Error::Message(format!("Failed to parse attributes: {:?}", e)))
            .map(|attributes| attributes.0)
    }
}

impl Parser<proc_macro::TokenStream> for AttributesParser {
    type Output = Attributes;
    fn parse(&self, token_stream: proc_macro::TokenStream) -> Result<Self::Output> {
        self.parse(proc_macro2::TokenStream::from(token_stream))
    }
}

impl Parser<syn::AttributeArgs> for AttributesParser {
    type Output = Attributes;
    fn parse(&self, attribute_args: syn::AttributeArgs) -> Result<Self::Output> {
        let attributes = attribute_args
            .iter()
            .map(|nested_meta| AttributeParser.parse(nested_meta.clone()).expect("Failed to parse nested meta."))
            .collect();
        Ok(Self::Output { attributes })
    }
}

impl Parser<syn::MetaList> for AttributesParser {
    type Output = Attributes;
    fn parse(&self, input: syn::MetaList) -> Result<Self::Output> {
        Ok(Self::Output {
            attributes: input
                .nested
                .into_iter()
                .map(|nested_meta| AttributeParser.parse(nested_meta).expect("Failed to parse nested meta."))
                .collect(),
        })
    }
}


impl ToTokens for Attributes {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let length = self.attributes.len();
        for (index, attribute) in self.attributes.iter().enumerate() {
            let attribute = attribute.to_token_stream();
            tokens.append_all(quote! { #attribute });
            if index != length - 1 {
                tokens.append_all(quote! { , });
            }
        }
    }
}

impl ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Attribute::Literal(literal) => {
                let literal = literal.to_token_stream();
                tokens.append_all(quote! {#literal})
            }
            Attribute::Named(_, _) => panic!("Named variant should only be used inside groups"),
            Attribute::Group(identifier, group) => {
                let mut attributes = proc_macro2::TokenStream::new();
                group
                    .attributes
                    .clone()
                    .into_iter()
                    .enumerate()
                    .for_each(|x| {
                        if let (index, Attribute::Named(identifier, lit)) = x {
                            let name = Identifier::new(&identifier.name).to_token_stream();
                            let lit = lit.to_token_stream();
                            attributes.append_all(quote! {#name = #lit});
                            if index + 1 < group.attributes.len() {
                                attributes.append_all(quote! {, })
                            }
                        } else {
                            panic!("Group contains Non Named variant")
                        }
                    });

                let identifier = identifier.to_token_stream();
                tokens.append_all(quote! {#identifier(#attributes)})
            }
        }
    }
}

// TODO: Can we remove this?
struct LigenAttributes(pub Attributes);

impl syn::parse::Parse for LigenAttributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut metas: Vec<Attribute> = Vec::new();

        while !input.is_empty() {
            let value = AttributeParser.parse(input.parse::<syn::NestedMeta>()?).expect("Failed to parse attribute");
            metas.push(value);
            if !input.is_empty() {
                input.parse::<syn::Token![,]>()?;
            }
        }
        Ok(LigenAttributes(Attributes::from(metas)))
    }
}

#[cfg(test)]
mod test {
    use ligen_ir::{Attribute, Attributes, Identifier, Literal};
    use quote::quote;
    use ligen_parsing::Parser;
    use crate::macro_attributes::attributes::{AttributeParser, AttributesParser};
    use crate::prelude::*;

    #[test]
    fn attribute_literal() -> Result<()> {
        let args: syn::NestedMeta = syn::parse_quote!("C");
        let attr: Attribute = AttributeParser.parse(args)?;
        assert_eq!(attr, Attribute::Literal(Literal::String(String::from("C"))));
        Ok(())
    }

    #[test]
    fn attribute_named() -> Result<()> {
        let args: syn::NestedMeta = syn::parse_quote!(int = "sized");
        let attr: Attribute = AttributeParser.parse(args)?;
        assert_eq!(
            attr,
            Attribute::Named(
                Identifier::new("int"),
                Literal::String(String::from("sized"))
            )
        );
        Ok(())
    }

    #[test]
    fn get_literal() -> Result<()> {
        let args: syn::NestedMeta = syn::parse_quote!(
            c(
                marshal_as(
                    name = "hello",
                    uuid = 5
                ),
                int = "sized"
            )
        );
        let attribute: Attribute = AttributeParser.parse(args)?;
        let attributes: Attributes = attribute.into();
        assert_eq!(attributes.get_literal_from_path(vec!["c", "int"]), Some(&Literal::String("sized".into())));
        assert_eq!(attributes.get_literal_from_path(vec!["c", "marshal_as", "name"]), Some(&Literal::String("hello".into())));
        assert_eq!(attributes.get_literal_from_path(vec!["c", "marshal_as", "uuid"]), Some(&Literal::Integer(5)));
        Ok(())
    }

    #[test]
    fn attribute_group() -> Result<()> {
        let args: syn::NestedMeta = syn::parse_quote!(C(int = "sized"));
        let attr: Attribute = AttributeParser.parse(args)?;
        assert_eq!(
            attr,
            Attribute::Group(
                Identifier::new("C"),
                Attributes {
                    attributes: vec![Attribute::Named(
                        Identifier::new("int"),
                        Literal::String(String::from("sized"))
                    )]
                }
            )
        );
        Ok(())
    }

    #[test]
    fn parse_attributes() -> Result<()> {
        assert_eq!(
            Attributes {
                attributes: vec![Attribute::Group(
                    Identifier::new("c"),
                    Attributes {
                        attributes: vec![Attribute::Named(
                            Identifier::new("int"),
                            Literal::String(String::from("sized"))
                        )]
                    }
                )]
            },
            AttributesParser.parse(quote! {c(int = "sized")})?
        );
        Ok(())
    }
}

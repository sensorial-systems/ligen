mod attribute;
pub use attribute::*;

use ligen_ir::{Attribute, Attributes, Identifier};
use crate::prelude::*;
use syn::parse::{ParseStream, Parse};
use syn::{parse2, Token};
use ligen_parsing::Parser;

pub struct AttributesParser;

impl Parser<Vec<syn::Attribute>> for AttributesParser {
    type Output = Attributes;
    fn parse(&self, in_attributes: Vec<syn::Attribute>) -> Result<Self::Output> {
        let mut attributes = Vec::new();
        for attribute in in_attributes {
            attributes.push(SynAttribute::from(attribute).try_into()?);
        }
        Ok(Self::Output { attributes })
    }
}

impl Parser<proc_macro2::TokenStream> for AttributesParser {
    type Output = Attributes;
    fn parse(&self, tokenstream: proc_macro2::TokenStream) -> Result<Self::Output> {
        Ok(parse2::<LigenAttributes>(tokenstream.clone()).map_err(|e| format!("Failed to parse Attributes: {:?}, input: {}", e, tokenstream.to_string()))?.0)
    }
}

impl Parser<proc_macro::TokenStream> for AttributesParser {
    type Output = Attributes;
    fn parse(&self, tokenstream: proc_macro::TokenStream) -> Result<Self::Output> {
        let tokenstream: TokenStream = tokenstream.into();
        self.parse(tokenstream)
    }
}

impl Parser<syn::AttributeArgs> for AttributesParser {
    type Output = Attributes;
    fn parse(&self, attribute_args: syn::AttributeArgs) -> Result<Self::Output> {
        let attributes = attribute_args
            .iter()
            .map(|nested_meta| Attribute::from(SynNestedMeta(nested_meta.clone())))
            .collect();
        Ok(Self::Output { attributes })
    }
}

impl ToTokens for Attributes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
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
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Attribute::Literal(literal) => {
                let literal = literal.to_token_stream();
                tokens.append_all(quote! {#literal})
            }
            Attribute::Named(_, _) => panic!("Named variant should only be used inside groups"),
            Attribute::Group(identifier, group) => {
                let mut attributes = TokenStream::new();
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

pub struct LigenAttributes(pub Attributes);

impl Parse for LigenAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut metas: Vec<Attribute> = Vec::new();

        while !input.is_empty() {
            let value = Attribute::from(SynNestedMeta(input.parse()?));
            metas.push(value);
            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }
        Ok(LigenAttributes(Attributes::from(metas)))
    }
}

#[cfg(test)]
mod test {
    use ligen_ir::{Attribute, Attributes, Identifier, Literal};
    use quote::quote;
    use syn::NestedMeta;
    use ligen_parsing::Parser;
    use crate::macro_attributes::attributes::AttributesParser;
    use crate::prelude::SynNestedMeta;

    #[test]
    fn attribute_literal() {
        let args: NestedMeta = syn::parse_quote!("C");
        let attr: Attribute = SynNestedMeta(args).into();
        assert_eq!(attr, Attribute::Literal(Literal::String(String::from("C"))))
    }

    #[test]
    fn attribute_named() {
        let args: NestedMeta = syn::parse_quote!(int = "sized");
        let attr: Attribute = SynNestedMeta(args).into();
        assert_eq!(
            attr,
            Attribute::Named(
                Identifier::new("int"),
                Literal::String(String::from("sized"))
            )
        )
    }

    #[test]
    fn get_literal() {
        let args: NestedMeta = syn::parse_quote!(
            c(
                marshal_as(
                    name = "hello",
                    uuid = 5
                ),
                int = "sized"
            )
        );
        let attribute: Attribute = SynNestedMeta(args).into();
        let attributes: Attributes = attribute.into();
        assert_eq!(attributes.get_literal_from_path(vec!["c", "int"]), Some(&Literal::String("sized".into())));
        assert_eq!(attributes.get_literal_from_path(vec!["c", "marshal_as", "name"]), Some(&Literal::String("hello".into())));
        assert_eq!(attributes.get_literal_from_path(vec!["c", "marshal_as", "uuid"]), Some(&Literal::Integer(5)));
    }

    #[test]
    fn attribute_group() {
        let args: NestedMeta = syn::parse_quote!(C(int = "sized"));
        let attr: Attribute = SynNestedMeta(args).into();
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
        )
    }

    #[test]
    fn parse_attributes() {
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
            AttributesParser.parse(quote! {c(int = "sized")}).expect("Failed to parse Attributes")
        );
    }
}

use crate::ir::Attributes;
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use syn::parse2;
use crate::ir::{Attribute, Identifier, Literal};

const PREFIX: &'static str = "ligen_";

/// `ligen` entry-point called by `#[ligen]`.
pub fn ligen(args: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse2::<Attributes>(args).expect("Failed to parse Attributes");

    let mut attributes = TokenStream::new();

    let macro_attributes = args
        .attributes
        .into_iter()
        .map(to_ligen_macro);

    macro_attributes.for_each(|macro_attribute|
        attributes.append_all(quote! { #macro_attribute })
    );

    let tokenstream = quote! {
        #attributes
        #item
    };

    tokenstream
}

/// Convert Attribute to a Ligen Macro attribute
pub fn to_ligen_macro(attribute: Attribute) -> Attribute {
    match attribute {
        Attribute::Literal(literal) => {
            Attribute::Literal(Literal::String(format!("{}{}", PREFIX, literal)))
        }
        Attribute::Named(ident, lit) => Attribute::Named(ident, lit),
        Attribute::Group(ident, group) => Attribute::Group(
            Identifier::new(format!("{}{}", PREFIX, ident.name).as_str()),
            Attributes {
                attributes: group
                    .attributes
                    .into_iter()
                    .filter_map(|x| {
                        if let Attribute::Named(ident, lit) = x {
                            Some(Attribute::Named(ident, lit))
                        } else {
                            None
                        }
                    })
                    .collect(),
            },
        ),
    }
}


#[cfg(test)]
mod test {
    use quote::quote;

    use crate::ligen;
    use proc_macro2::TokenStream;
    use quote::*;
    use syn::parse_quote::parse;

    fn extract_struct_attributes_and_item(
        item_impl: &TokenStream,
    ) -> Result<(TokenStream, TokenStream), &'static str> {
        let mut item: syn::ItemStruct = parse(item_impl.clone());
        let ligen_attribute = item
            .attrs
            .iter()
            .find(|attr| attr.path.to_token_stream().to_string() == "ligen")
            .expect("Couldn't find ligen");
        let meta = ligen_attribute.parse_meta().expect("Couldn't parse Meta");
        if let syn::Meta::List(ref meta_list) = meta {
            item.attrs.clear();
            Ok((meta_list.nested.to_token_stream(), item.to_token_stream()))
        } else {
            Err("Couldn't find attribute.")
        }
    }

    fn extract_impl_attributes_and_item(
        item_impl: &TokenStream,
    ) -> Result<(TokenStream, TokenStream), &'static str> {
        let mut item: syn::ItemImpl = parse(item_impl.clone());
        let ligen_attribute = item
            .attrs
            .iter()
            .find(|attr| attr.path.to_token_stream().to_string() == "ligen")
            .expect("Couldn't find ligen");
        let meta = ligen_attribute.parse_meta().expect("Couldn't parse Meta");
        if let syn::Meta::List(ref meta_list) = meta {
            item.attrs.clear();
            Ok((meta_list.nested.to_token_stream(), item.to_token_stream()))
        } else {
            Err("Couldn't find attribute.")
        }
    }

    #[test]
    fn item_struct() {
        let input = quote! {
            #[ligen(c, cpp)]
            struct Object {}
        };

        let expected = quote! {
            #[ligen_c]
            #[ligen_cpp]
            struct Object {}
        };


        let (attributes, item) = extract_struct_attributes_and_item(&input)
            .expect("Couldn't extract attributes and item.");
        let token_stream = ligen(attributes, item);
        assert_eq!(token_stream.to_string(), expected.to_string());
    }

    #[test]
    fn item_impl() {
        let input = quote! {
            #[ligen(c(integer = "sized"), cpp(float = "sized"))]
            impl Object {}
        };

        let expected = quote! {
            #[ligen_c(integer = "sized")]
            #[ligen_cpp(float = "sized")]
            impl Object {}
        };

        let (attributes, item) = extract_impl_attributes_and_item(&input)
            .expect("Couldn't extract attributes and item.");
        let token_stream = ligen(attributes, item);
        assert_eq!(token_stream.to_string(), expected.to_string());
    }
}

//! proc-macro entrypoint.

use crate::ir::Attributes;
use crate::ir::{Attribute, Identifier, Literal};
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use std::convert::TryFrom;

pub mod context;
pub use context::*;

const PREFIX: &'static str = "ligen_";

/// `ligen` entry-point called by `#[ligen]`.
pub fn ligen(_context: Context, args: TokenStream, item: TokenStream) -> TokenStream {
    let args = Attributes::try_from(args).expect("Failed to parse Attributes.");

    let attributes = args.attributes.into_iter().map(to_ligen_macro).fold(
        TokenStream::new(),
        |mut attributes, macro_attribute| {
            attributes.append_all(quote! { #macro_attribute });
            attributes
        },
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
    use std::path::PathBuf;

    use quote::quote;

    use super::{Context, SourceFile, Arguments, BuildType};
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

    fn mock_context() -> Context {
        Context {
            source_file: SourceFile {
                is_real: true,
                path: PathBuf::from("test"),
            },
            arguments: Arguments {
                crate_name: "test".into(),
                build_type: BuildType::Debug,
                target_dir: PathBuf::from("test"),
                manifest_path: PathBuf::from("test"),
                workspace_path: None,
                workpace_member: None,
            }
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
        let token_stream = ligen(
            mock_context(),
            attributes,
            item,
        );
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
        let token_stream = ligen(
            mock_context(),
            attributes,
            item,
        );
        assert_eq!(token_stream.to_string(), expected.to_string());
    }
}

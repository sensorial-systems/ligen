#[cfg(test)]
mod test {
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
            #[ligen(C, CPP)]
            struct Object {

            }
        };

        let (attributes, item) = extract_struct_attributes_and_item(&input)
            .expect("Couldn't extract attributes and item.");
        let _token_stream = ligen(attributes, item);
    }

    #[test]
    fn item_impl() {
        let input = quote! {
            #[ligen(C, CPP)]
            impl Object {
                fn private_fn() -> Self {
                    Self {}
                }

                pub fn new() -> Object {
                    Self {}
                }
            }
        };

        let expected = quote! {
            #[ligen_C]
            #[ligen_CPP]
            impl Object {
                fn private_fn() -> Self {
                    Self {}
                }

                pub fn new() -> Object {
                    Self {}
                }
            }
        };

        let (attributes, item) = extract_impl_attributes_and_item(&input)
            .expect("Couldn't extract attributes and item.");
        let token_stream = ligen(attributes, item);
        assert_eq!(token_stream.to_string(), expected.to_string());
    }
}

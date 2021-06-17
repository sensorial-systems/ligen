use crate::ir::Attributes;
use proc_macro2::TokenStream;
use quote::TokenStreamExt;
use std::convert::TryFrom;


/// `ligen_package` macro function called by `ligen_package!()`
pub fn ligen_package(attributes: TokenStream) -> TokenStream {
    let attributes = Attributes::try_from(attributes).expect("Failed to parse Attributes.");

    let mut output = TokenStream::new();
    attributes.attributes
        .into_iter()
        .for_each(|attribute| output.append_all(attribute.to_package_tokens()));

    output
}
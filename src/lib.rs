//! This crate provides core functionalities for ligen.

#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unsafe_code)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]

pub mod ir;
mod prelude;
pub mod utils;

use crate::ir::Attributes;
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use syn::parse2;

/// `ligen` entry-point called by `#[ligen]`.
pub fn ligen(args: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse2::<Attributes>(args).expect("Failed to parse Attributes");

    let mut stream = TokenStream::new();

    let macro_attributes = args
        .attributes
        .iter()
        .map(|attribute| attribute.to_ligen_macro());

    macro_attributes.for_each(|macro_attribute| stream.append_all(quote! { #macro_attribute }));

    quote! {
        #stream
        #item
    }
}

#[cfg(test)]
mod test {
    use super::ligen;
    use quote::quote;

    #[test]
    fn ligen_main() {
        assert_eq!(
            quote! {
                #[ligen_c(int = "sized")]
                #[ligen_python]
                struct Test;
            }
            .to_string(),
            ligen(quote! {c(int = "sized"), python}, quote! {struct Test;}).to_string()
        );
    }
}

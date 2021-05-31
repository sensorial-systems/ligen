use std::env;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn ligen(args: TokenStream, input: TokenStream) -> TokenStream {
    ligen_core::ligen(Default::default(), args.into(), input.into()).into()
}

#[proc_macro]
pub fn ligen_package(args: TokenStream) -> TokenStream {
    ligen_core::ligen_package(args.into()).into()
}

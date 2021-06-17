use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn ligen(attributes: TokenStream, input: TokenStream) -> TokenStream {
    ligen_core::proc_macro::ligen(Default::default(), attributes.into(), input.into()).into()
}

#[proc_macro]
pub fn ligen_package(attributes: TokenStream) -> TokenStream {
    ligen_core::proc_macro::ligen_package(attributes.into()).into()
}

#[proc_macro]
pub fn proc_macro_attribute(attributes: TokenStream) -> TokenStream {
    ligen_core::proc_macro::proc_macro_wrapper(attributes.into(), true).into()
}

#[proc_macro]
pub fn proc_macro(attributes: TokenStream) -> TokenStream {
    ligen_core::proc_macro::proc_macro_wrapper(attributes.into(), false).into()
}

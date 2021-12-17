use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn ligen(_attributes: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro]
pub fn inner_ligen(_input: TokenStream) -> TokenStream {
    Default::default()
}
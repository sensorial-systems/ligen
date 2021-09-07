use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn ligen(_attributes: TokenStream, input: TokenStream) -> TokenStream {
    input
}

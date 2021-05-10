use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn ligen(args: TokenStream, input: TokenStream) -> TokenStream {
    ligen_core::ligen(args.into(), input.into()).into()
}

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
pub fn define_binding_generator(attributes: TokenStream) -> TokenStream {
    ligen_core::proc_macro::define_binding_generator(attributes.into()).into()
}

#[proc_macro]
pub fn define_project_generator(attributes: TokenStream) -> TokenStream {
    ligen_core::proc_macro::define_project_generator(attributes.into()).into()
}

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn ligen(attributes: TokenStream, input: TokenStream) -> TokenStream {
    ligen_core::r#macro::ligen(Default::default(), attributes.into(), input.into()).into()
}

#[proc_macro]
pub fn ligen_project(attributes: TokenStream) -> TokenStream {
    ligen_core::r#macro::ligen_project(attributes.into()).into()
}

#[proc_macro]
pub fn ligen_dependencies(attributes: TokenStream) -> TokenStream {
    ligen_core::r#macro::ligen_dependencies(attributes.into()).into()
}

#[proc_macro]
pub fn define_binding_generator(attributes: TokenStream) -> TokenStream {
    ligen_core::r#macro::define_binding_generator(attributes.into()).into()
}

#[proc_macro]
pub fn define_project_generator(attributes: TokenStream) -> TokenStream {
    ligen_core::r#macro::define_project_generator(attributes.into()).into()
}

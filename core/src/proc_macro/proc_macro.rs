//! proc_macro_attribute definition module.

use crate::ir::{Identifier, Path, Attributes};

use proc_macro2::TokenStream;
use quote::quote;
use std::convert::TryFrom;

fn get_parameters(attributes: TokenStream) -> (Identifier, Path) {
    let attributes = Attributes::try_from(attributes).expect("Couldn't parse attributes.");
    let function_identifier = attributes.get_named("name").expect("Procedural macro name not present. e.g.: name = \"ligen_cpp\"");
    let function_identifier = Identifier::new(function_identifier.to_string());
    let generator_path = attributes.get_named("generator").expect("Generator path not present. e.g.: generator = \"ligen_c_core::Generator\"");
    let generator_path: Path = generator_path.to_string().into();
    (function_identifier, generator_path)
}

// FIXME: Needs better doc, better name, cleanup and simplification.
/// Proc-macro wrapper.
pub fn proc_macro_wrapper(attributes: TokenStream, is_attribute: bool) -> TokenStream {
    let (function_identifier, generator_path) = get_parameters(attributes);

    // FIXME: THIS WHOLE SECTION IS HORRIBLE.
    let (proc_macro_type, implementation, output, function_signature, passthrough) = if is_attribute {
        let proc_macro_type = quote! { #[proc_macro_attribute] };
        let implementation = quote! { ir::Implementation::try_from(input).ok() };
        let output = quote! {
            let input: proc_macro2::TokenStream = input.into();
            let mut output = input.clone();
        };
        let function_signature = quote! {
            pub fn #function_identifier(attributes: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream
        };
        let passthrough = quote! {
            input
        };
        (proc_macro_type, implementation, output, function_signature, passthrough)
    } else {
        let proc_macro_type = quote! { #[proc_macro] };
        let implementation = quote! { None };
        let output = quote! {
            let mut output = proc_macro2::TokenStream::new();
        };
        let function_signature = quote! {
            pub fn #function_identifier(attributes: proc_macro::TokenStream) -> proc_macro::TokenStream
        };
        let passthrough = quote! {
            proc_macro2::TokenStream::new()
        };
        (proc_macro_type, implementation, output, function_signature, passthrough)
    };

    quote! {
        /// Generator proc-macro entry-point.
        #[cfg(cargo_ligen)]
        #proc_macro_type
        #function_signature {
            let source_file = proc_macro::Span::call_site().source_file();
            let source_file = ligen_core::generator::SourceFile {
                is_real: source_file.is_real(),
                path: source_file.path(),
            };
            let arguments =
                ligen_core::generator::Arguments::from_env().expect("Failed to get the arguments");
            let context = Context {
                source_file,
                arguments,
            };
            let attributes: proc_macro2::TokenStream = attributes.into();
            let attributes = ligen_core::ir::Attributes::try_from(attributes).expect("Couldn't get attributes.");
            let generator = #generator_path::new(&context, &attributes);
            #output;
            let implementation: Option<ir::Implementation> = #implementation;
            let implementation = implementation.map(|mut implementation| {
                let id = implementation.self_.clone();
                let mut lower_case_id = id.clone();
                lower_case_id.name = lower_case_id.name.to_lowercase();
                implementation.replace_identifier(&ir::Identifier::from("Self"), &id);
                implementation.replace_identifier(&ir::Identifier::from("self"), &lower_case_id);
                implementation
            });
            output.append_all(generator.generate(&context, implementation.as_ref()));
            output.into()
        }

        /// Generator proc-macro entry-point.
        #[cfg(not(cargo_ligen))]
        #[proc_macro_attribute]
        #function_signature {
            #passthrough
        }
    }
}

//! proc_macro_attribute definition module.

use proc_macro2::TokenStream;
use quote::quote;
use crate::ir::Identifier;
use crate::ir::Attributes;
use crate::ir::Path;
use std::convert::TryFrom;

/// Helper to create proc_macro_attribute for the generators.
pub fn proc_macro_attribute(attributes: TokenStream) -> TokenStream {
    let attributes = Attributes::try_from(attributes).expect("Couldn't parse attributes.");
    let function_identifier = attributes.get_named("name").expect("Procedural macro name not present. e.g.: name = \"ligen_cpp\"");
    let function_identifier = Identifier::new(function_identifier.to_string());
    let generator_path = attributes.get_named("generator").expect("Generator path not present. e.g.: generator = \"ligen_c_core::Generator\"");
    let generator_path: Path = generator_path.to_string().into();
    quote! {
        /// Generator proc-macro entry-point.
        #[cfg(cargo_ligen)]
        #[proc_macro_attribute]
        pub fn #function_identifier(attributes: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
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
            let input: proc_macro2::TokenStream = input.into();
            let mut output = input.clone();
            if let Ok(mut implementation) = ir::Implementation::try_from(input) {
                let id = implementation.self_.clone();
                let mut lower_case_id = id.clone();
                lower_case_id.name = lower_case_id.name.to_lowercase();
                implementation.replace_identifier(&ir::Identifier::from("Self"), &id);
                implementation.replace_identifier(&ir::Identifier::from("self"), &lower_case_id);
                output.append_all(generator.generate(&context, &implementation));
            }
            output.into()
        }

        /// Generator proc-macro entry-point.
        #[cfg(not(cargo_ligen))]
        #[proc_macro_attribute]
        pub fn #function_identifier(_args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            input
        }

    }
}
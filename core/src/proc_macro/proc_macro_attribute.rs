//! proc_macro_attribute definition module.

use proc_macro2::TokenStream;
use quote::quote;

/// Helper to create proc_macro_attribute for the generators.
pub fn proc_macro_attribute(_attributes: TokenStream) -> TokenStream {
    quote! {
        /// Generator proc-macro entry-point.
        #[cfg(cargo_ligen)]
        #[proc_macro_attribute]
        pub fn ligen_c(attributes: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            let source_file = proc_macro::Span::call_site().source_file();
            let source_file = ligen_c_core::SourceFile {
                is_real: source_file.is_real(),
                path: source_file.path(),
            };
            let arguments =
                ligen_core::proc_macro::Arguments::from_env().expect("Failed to get the arguments");
            let context = ligen_c_core::Context {
                source_file,
                arguments,
            };
            let attributes: proc_macro2::TokenStream = attributes.into();
            let attributes = ligen_core::ir::Attributes::try_from(attributes).expect("Couldn't get attributes.");
            let generator = ligen_c_core::generator::Generator::new(&context, &attributes);
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
        pub fn ligen_c(_args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            input
        }

    }
}
//! proc_macro definition module.

use proc_macro2::TokenStream;
use quote::quote;

/// Helper to create proc_macro for the generators.
pub fn proc_macro(_attributes: TokenStream) -> TokenStream {
    // FIXME: Hardcoded values.
    quote! {
        #[cfg(cargo_ligen)]
        #[proc_macro]
        /// Project generator proc-macro entry-point.
        pub fn ligen_c_package(attributes: proc_macro::TokenStream) -> proc_macro::TokenStream {
            use ligen_core::generator::Generator;
            use std::convert::TryFrom;
            use quote::TokenStreamExt;
            use ligen_core::ir;
            use ligen_core::ir::processing::ReplaceIdentifier;

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
            ligen_c_core::ligen_c_package(context, attributes.into()).into()
        }

        #[cfg(not(cargo_ligen))]
        #[proc_macro]
        /// Project generator proc-macro entry-point.
        pub fn ligen_c_package(_args: proc_macro::TokenStream) -> proc_macro::TokenStream {
            TokenStream::new()
        }

    }
}
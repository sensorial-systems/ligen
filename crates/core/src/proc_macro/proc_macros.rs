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

/// Used to define a new binding generator macro.
pub fn define_binding_generator(attributes: TokenStream) -> TokenStream {
    let (function_identifier, generator_path) = get_parameters(attributes);
    let function_signature = quote! {
        pub fn #function_identifier(attributes: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream
    };

    quote! {
        /// Generator proc-macro entry-point.
        #[cfg(cargo_ligen)]
        #[proc_macro_attribute]
        #function_signature {
            use ligen::proc_macro::prelude::*;
            let context = Context::current().expect("Couldn't get context.");
            let attributes = attributes.try_into().expect("Failed to parse attributes.");
            let implementation = input.clone().try_into().ok();
            let mut output: TokenStream = input.into();
            let generator = #generator_path::new(&context, &attributes);
            let generated = generator.generate(&context, implementation.as_ref()).expect("Generator failed.");
            output.append_all(generated);
            output.into()
        }

        /// Generator proc-macro entry-point.
        #[cfg(not(cargo_ligen))]
        #[proc_macro_attribute]
        #function_signature {
            input
        }
    }
}

/// Used to define a new project generator macro.
pub fn define_project_generator(attributes: TokenStream) -> TokenStream {
    let (function_identifier, generator_path) = get_parameters(attributes);
    let function_signature = quote! {
        pub fn #function_identifier(attributes: proc_macro::TokenStream) -> proc_macro::TokenStream
    };

    quote! {
        /// Generator proc-macro entry-point.
        #[cfg(cargo_ligen)]
        #[proc_macro]
        #function_signature {
            use ligen::proc_macro::prelude::*;
            let context = Context::current().expect("Couldn't get context.");
            let attributes = attributes.try_into().expect("Failed to parse attributes.");
            let implementation = None;
            let generator = #generator_path::new(&context, &attributes);
            let generated = generator.generate(&context, implementation.as_ref()).expect("Generator failed.");
            let output = generated;
            output.into()
        }

        /// Generator proc-macro entry-point.
        #[cfg(not(cargo_ligen))]
        #[proc_macro]
        #function_signature {
            input
        }
    }
}

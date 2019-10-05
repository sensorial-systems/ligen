extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input};

use ligen_core::{Object, Generator, Attribute};

#[proc_macro_attribute]
pub fn ligen(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as syn::AttributeArgs);
    let attribute = Attribute::parse_args(&args);

    let parsed : syn::Result<syn::ItemImpl> = syn::parse(input.clone());

    let output = match &parsed {
        Result::Ok(input) => {
            let object = Object::parse(input.clone());

            let generator = Generator::new(&attribute);
            match generator {
                Ok(generator) => generator.generate(&object),
                Err(err) => eprintln!("{}", err)
            }

            let output = TokenStream::from(quote!{
                #input
                #object
            });
            output
        },
        Result::Err(_) => {
            input
        }
    };

    output
}

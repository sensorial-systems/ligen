extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input};

use ligen_core::{Object};
use ligen_core::{Generator};

#[proc_macro_attribute]
pub fn ligen(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as syn::AttributeArgs);
    let input = parse_macro_input!(input as syn::ItemImpl); // same as let input : syn::ItemImpl = syn::parse(input).unwrap();
    let object = Object::parse(input.clone());

    for arg in args {
        let generator = Generator::new(&arg);
        match generator {
            Ok(generator) => {
                generator.generate(&object);
            },
            Err(err) => println!("{}", err),
        }
    }

    let output = TokenStream::from(quote!{
        #input
        #object
    });
    output
}

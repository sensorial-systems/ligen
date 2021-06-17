//! proc-macros utils.

use proc_macro2::TokenStream;
use crate::ir::{Identifier, Path, Attributes};
use std::convert::TryFrom;

pub fn get_parameters(attributes: TokenStream) -> (Identifier, Path) {
    let attributes = Attributes::try_from(attributes).expect("Couldn't parse attributes.");
    let function_identifier = attributes.get_named("name").expect("Procedural macro name not present. e.g.: name = \"ligen_cpp\"");
    let function_identifier = Identifier::new(function_identifier.to_string());
    let generator_path = attributes.get_named("generator").expect("Generator path not present. e.g.: generator = \"ligen_c_core::Generator\"");
    let generator_path: Path = generator_path.to_string().into();
    (function_identifier, generator_path)
}
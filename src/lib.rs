mod object;
mod method;
mod identifier;
mod input;
mod output;
mod ty;
mod attribute;
mod literal;

mod generator;

pub use literal::Literal;
pub use attribute::{Attribute, Attributes, LiteralConverter};
pub use object::Object;
pub use method::Method;
pub use identifier::Identifier;
pub use input::{Input, Inputs};
pub use output::Output;
pub use ty::{Type, Reference, TypeModifier};

pub use generator::{Generator, Files, File};

pub fn get_build_path() -> String {
    let profile = if cfg!(debug_assertions) { "debug" } else { "release" };
    format!("./target/{}", profile)
}

pub fn get_path() -> String {
    format!("{}/ligen", get_build_path())
}

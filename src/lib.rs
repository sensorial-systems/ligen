mod object;
mod method;
mod identifier;
mod input;
mod output;
mod ty;

mod generator;

pub use object::Object;
pub use method::Method;
pub use identifier::Identifier;
pub use input::{Input, Inputs};
pub use output::Output;
pub use ty::Type;

pub use generator::{Generator, Files, File};

pub fn get_path() -> String {
    let profile = if cfg!(debug_assertions) { "debug" } else { "release" };
    format!("./target/{profile}/ligen", profile = profile)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

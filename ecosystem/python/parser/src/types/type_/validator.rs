use crate::prelude::*;
use ligen::{ir::{Type, Path}, parser::{ParserConfig, ParserConfigGet, Validator}};

#[derive(Default)]
pub struct TypeValidator {}

impl TypeValidator {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Validator for TypeValidator {
    type Input = Type;
    fn validate(&self, type_: &mut Type, config: &ParserConfig) -> Result<()> {
        let name = type_.path.last().identifier.name.as_str();
        if config.get(Path::from("ligen::python::as-opaque").join(name)).is_some() {
            println!("{}", type_.path);
            *type_ = Type::opaque();
        }
        Ok(())
    }
}

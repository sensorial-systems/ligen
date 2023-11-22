use crate::prelude::*;
use ligen::{ir::{Type, Path}, parser::{ParserConfig, ParserConfigGet}};

#[derive(Default)]
pub struct TypeValidator {}

impl TypeValidator {
    pub fn new() -> Self {
        Default::default()
    }
}

impl TypeValidator {
    pub fn validate(&self, type_: &mut Type, config: &ParserConfig) -> Result<()> {
        let name = type_.path.last().identifier.name.as_str();
        // TODO: Move it to a validation step. It's hard to find it here.
        if config.get(Path::from("ligen::python::as-opaque").join(name)).is_some() {
            *type_ = Type::opaque();
        }
        Ok(())
    }
}
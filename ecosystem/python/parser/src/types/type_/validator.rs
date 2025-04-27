use crate::prelude::*;
use ligen::ir::{Type, Path};

#[derive(Default)]
pub struct TypeValidator {}

impl TypeValidator {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Validator<Type> for TypeValidator {
    fn validate(&self, type_: &mut Type, config: &Config) -> Result<()> {
        let name = type_.path.last().identifier.name.as_str();
        if config.get(Path::from("ligen::python::as-opaque").join(name)).is_some() {
            *type_ = Type::opaque();
        }
        Ok(())
    }
}

use crate::prelude::*;
use is_tree::prelude::*;
use ligen::{ir::{Library, Type}, parser::ParserConfig};
use crate::types::type_::TypeValidator;

#[derive(Default)]
pub struct LibraryValidator {
    type_validator: TypeValidator,
}

impl LibraryValidator {
    pub fn new() -> Self {
        Default::default()
    }
}

impl LibraryValidator {
    pub fn validate(&self, library: &mut Library, config: &ParserConfig) -> Result<()> {
        library.iter_type_mut::<Type>().for_each(|type_| {
            self.type_validator.validate(type_, config).unwrap();
        });
        Ok(())
    }
}

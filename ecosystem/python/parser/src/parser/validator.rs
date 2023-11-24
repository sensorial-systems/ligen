use crate::{prelude::*, function::method::validator::MethodValidator, interface::validator::InterfaceValidator};
use is_tree::prelude::*;
use ligen::{ir::{Library, Type, Method, Interface}, parser::ParserConfig};
use crate::types::type_::TypeValidator;

#[derive(Default)]
pub struct LibraryValidator {
    type_validator: TypeValidator,
    method_validator: MethodValidator,
    interface_validator: InterfaceValidator
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
        library.iter_type_mut::<Interface>().for_each(|interface| {
            self.interface_validator.validate(interface, config).unwrap();
        });
        library.iter_type_mut::<Method>().for_each(|method| {
            self.method_validator.validate(method, config).unwrap();
        });
        Ok(())
    }
}

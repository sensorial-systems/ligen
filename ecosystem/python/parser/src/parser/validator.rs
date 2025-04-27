use crate::{prelude::*, function::method::validator::MethodValidator, interface::validator::InterfaceValidator};
use ligen::ir::Library;
use crate::types::type_::TypeValidator;

#[derive(Default)]
pub struct LibraryValidator {
    _type_validator: TypeValidator,
    _method_validator: MethodValidator,
    _interface_validator: InterfaceValidator
}

impl LibraryValidator {
    pub fn new() -> Self {
        Default::default()
    }
}

impl LibraryValidator {
    pub fn validate(&self, _library: &mut Library, _config: &Config) -> Result<()> {
        // library.iter_type::<Type>().for_each(|type_| {
        //     self.type_validator.validate(type_, config).unwrap();
        // });
        // library.iter_type_mut::<Interface>().for_each(|interface| {
        //     self.interface_validator.validate(interface, config).unwrap();
        // });
        // library.iter_type_mut::<Method>().for_each(|method| {
        //     self.method_validator.validate(method, config).unwrap();
        // });
        Ok(())
    }
}

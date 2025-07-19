use crate::prelude::*;
use ligen::idl::{Interface, Function, Identifier};

#[derive(Default)]
pub struct InterfaceValidator {}

impl InterfaceValidator {
    pub fn new() -> Self {
        Default::default()
    }

    fn validate_constructor(&self, interface: &mut Interface, _config: &Config) -> Result<()> {
        let indices = interface.methods.iter().enumerate().filter_map(|(i, method)| {
            if method.identifier == "__init__" {
                Some(i)
            } else {
                None
            }
        }).collect::<Vec<_>>();
        for index in indices {
            let method = interface.methods.remove(index);
            let mut function = Function::from(method);
            function.identifier = Identifier::from("new");
            function.inputs.remove(0);
            function.output = Some(interface.identifier.clone().into());
            interface.functions.push(function);
        }
        Ok(())
    }
}

impl Validator<Interface> for InterfaceValidator {
    fn validate(&self, interface: &mut Interface, config: &Config) -> Result<()> {
        self.validate_constructor(interface, config)?;
        Ok(())
    }
}
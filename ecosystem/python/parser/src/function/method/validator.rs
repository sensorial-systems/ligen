use crate::prelude::*;
use ligen::parser::prelude::*;
use ligen::ir::Method;

#[derive(Default)]
pub struct MethodValidator {

}

impl MethodValidator {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Validator for MethodValidator {
    type Input = Method;
    fn validate(&self, _method: &mut Method, _config: &Config) -> Result<()> {
        Ok(())
    }
}
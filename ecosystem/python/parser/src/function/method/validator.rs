use crate::prelude::*;
use ligen::parser::ParserConfig;
use ligen::ir::Method;

#[derive(Default)]
pub struct MethodValidator {

}

impl MethodValidator {
    pub fn new() -> Self {
        Default::default()
    }
}

impl MethodValidator {
    pub fn validate(&self, _method: &mut Method, _config: &ParserConfig) -> Result<()> {
        Ok(())
    }
}
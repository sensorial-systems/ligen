use std::rc::Weak;

use ligen_transformer::prelude::*;
use ligen_ir::Type;

use crate::WgslPathGenerator;

pub struct WgslTypeGenerator {
    pub path_generator: Weak<WgslPathGenerator>,
}

impl WgslTypeGenerator {
    pub fn new(path_generator: Weak<WgslPathGenerator>) -> Self {
        Self { path_generator }
    }
}

impl Generator<&Type, String> for WgslTypeGenerator {
    fn generate(&self, type_: &Type, config: &Config) -> Result<String> {
        self.path_generator.upgrade().ok_or(Error::Message("Path generator not found".to_string()))?.generate(&type_.path, config)
    }
}
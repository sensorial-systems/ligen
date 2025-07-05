use std::rc::Rc;

use ligen_transformer::prelude::*;
use ligen_ir::Parameter;

use crate::{WgslIdentifierGenerator, WgslPathGenerator};

pub struct WgslParameterGenerator {
    pub path_generator: Rc<WgslPathGenerator>,
    pub identifier_generator: WgslIdentifierGenerator,
}

impl Default for WgslParameterGenerator {
    fn default() -> Self {
        let identifier_generator = Default::default();
        let path_generator = WgslPathGenerator::new();
        Self { identifier_generator, path_generator }
    }
}

impl Generator<&Parameter, String> for WgslParameterGenerator {
    fn generate(&self, parameter: &Parameter, config: &Config) -> Result<String> {
        let mut result = String::new();
        result.push_str(&self.identifier_generator.generate(&parameter.identifier, config)?);
        result.push_str(&format!(": {}", self.path_generator.path_segment_generator.type_generator.generate(&parameter.type_, config)?));
        Ok(result)
    }
}
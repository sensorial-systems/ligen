use crate::prelude::*;
use crate::generator_config::GeneratorConfig;

pub trait Generator<Input> {
    type Output;
    fn generate(&self, input: &Input, config: &GeneratorConfig) -> Result<Self::Output>;
}
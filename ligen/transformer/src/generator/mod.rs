mod file_generator;

use crate::prelude::*;

pub use file_generator::*;

pub trait Generator<Input> {
    type Output;
    fn generate(&self, input: &Input, config: &Config) -> Result<Self::Output>;
}
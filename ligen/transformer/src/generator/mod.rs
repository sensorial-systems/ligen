mod file_generator;

use crate::prelude::*;

pub use file_generator::*;

pub use crate::Config;

pub trait Generator<Input, Output> {
    fn generate(&self, input: Input, config: &Config) -> Result<Output>;
}

#[async_trait]
pub trait AsyncGenerator<Input, Output> {
    async fn generate(&self, input: Input, config: &Config) -> Result<Output>;
}

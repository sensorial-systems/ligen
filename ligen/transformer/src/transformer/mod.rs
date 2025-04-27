use crate::prelude::*;

pub trait Transformer<Input, Output> {
    fn transform(&self, input: Input, config: &Config) -> Result<Output>;

    fn name(&self) -> &str {
        "Transformer"
    }

    fn config(&self) -> Config {
        Default::default()
    }
}
pub mod universal;

use crate::prelude::*;

pub trait Parser<Input> {
    type Output;
    fn parse(&self, input: Input, config: &Config) -> Result<Self::Output>;
    fn name(&self) -> &str {
        "Parser"
    }

    fn config(&self) -> Config {
        Default::default()
    }
}

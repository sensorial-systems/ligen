use crate::prelude::*;

pub trait Validator<Input> {
    fn validate(&self, input: &mut Input, config: &Config) -> Result<()>;
}
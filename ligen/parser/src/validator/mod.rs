use crate::prelude::*;

pub trait Validator {
    type Input;
    fn validate(&self, input: &mut Self::Input, config: &Config) -> Result<()>;
}
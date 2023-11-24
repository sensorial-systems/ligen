use crate::{prelude::*, ParserConfig};

pub trait Validator {
    type Input;
    fn validate(&self, input: &mut Self::Input, config: &ParserConfig) -> Result<()>;
}
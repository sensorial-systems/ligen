pub mod universal;

use ligen_common::{Error, Result};

pub trait Parser<Input> {
    type Output;
    fn parse(&self, input: Input) -> Result<Self::Output>;
    fn parse_symbols(&self, _input: Input) -> Result<Self::Output> {
        Err(Error::Message("Not implemented".to_string()))
    }
}
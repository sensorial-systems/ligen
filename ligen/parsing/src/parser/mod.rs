pub mod universal;

use ligen_common::Result;

pub trait Parser<Input> {
    type Output;
    fn parse(&self, input: Input) -> Result<Self::Output>;
}

pub trait DynamicParser<Input> {
    type Output;
    fn get_parser(&self) -> &dyn Parser<Input, Output = Self::Output>;
}

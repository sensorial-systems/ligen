pub mod universal;

use ligen_common::Result;

pub trait Parser<Input> {
    type Output;
    fn parse(&self, input: Input) -> Result<Self::Output>;
}

pub mod universal;
pub mod config;

pub use config::*;

use ligen_common::Result;

pub trait Parser<Input> {
    type Output;
    fn parse(&self, input: Input, config: &ParserConfig) -> Result<Self::Output>;
    fn name(&self) -> &str {
        "Parser"
    }
    fn config(&self) -> ParserConfig {
        Default::default()
    }
}

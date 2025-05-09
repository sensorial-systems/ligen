pub mod universal;

use crate::prelude::*;

pub trait Parser<Output> {
    fn parse(&self, input: impl AsRef<str>, config: &Config) -> Result<Output>;
    fn name(&self) -> &str {
        "Parser"
    }

    fn config(&self) -> Config {
        Default::default()
    }
}

impl<Output, P: Parser<Output>> Transformer<String, Output> for P {
    fn transform(&self, input: String, config: &Config) -> Result<Output> {
        self.parse(input, config)
    }
}

impl<Output, P: Parser<Output>> Transformer<&str, Output> for P {
    fn transform(&self, input: &str, config: &Config) -> Result<Output> {
        self.parse(input, config)
    }
}

#[async_trait]
pub trait AsyncParser<Output> {
    async fn parse(&self, input: &str, config: &Config) -> Result<Output>;

    fn name(&self) -> &str {
        "Async Parser"
    }

    fn config(&self) -> Config {
        Default::default()
    }
}

#[async_trait]
impl<Output, P> AsyncTransformer<String, Output> for P
where
    P: AsyncParser<Output> + Send + Sync,
{
    async fn transform(&self, input: String, config: &Config) -> Result<Output> {
        self.parse(&input, config).await
    }
}
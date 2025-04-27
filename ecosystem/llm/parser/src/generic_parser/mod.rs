use ligen_ir::prelude::anyhow::Context;
use ligen_transformer::prelude::*;

use llm::builder::{LLMBackend, LLMBuilder};
use llm::chat::{ChatMessage, StructuredOutputFormat};
use llm::LLMProvider;

use crate::schema::JsonSchema;
use crate::TypeDescriptor;

pub struct GeneralLlmParser<T: TypeDescriptor> {
    llm: Box<dyn LLMProvider>,
    phantom: std::marker::PhantomData<T>,
}

impl<T: TypeDescriptor> GeneralLlmParser<T> {
    pub fn new() -> Result<Self> {
        let api_key = dotenv::var("OPENAI_API_KEY").context("OPENAI_API_KEY is not set")?;

        let mut schema = JsonSchema::new::<T>()?;
        schema.enforce_openai_subset();

        let structured_output = StructuredOutputFormat {
            description: Some(T::description()),
            name: T::name(),
            schema: Some(schema.value),
            strict: Some(true),
        };

        let llm = LLMBuilder::new()
            .system(format!("You are an universal {name} parser. You will be given a {input_description} and you will need to parse it into a {name} struct.", name = T::name(), input_description = T::input_description().to_lowercase()))
            .backend(LLMBackend::OpenAI)
            .api_key(api_key)
            .model("gpt-4o")
            .temperature(0.7)
            .stream(false)
            .schema(structured_output)
            .build()
            .context("Failed to build LLM (OpenAI)")?;

        let phantom = Default::default();
        Ok(Self { llm, phantom })
    }
}

#[async_trait]
impl<T: TypeDescriptor> AsyncParser<T> for GeneralLlmParser<T>
where Self: Send + Sync
{
    async fn parse(&self, input: &str, _config: &Config) -> Result<T> {
        let messages = vec![
            ChatMessage::user()
                .content(input)
                .build(),
        ];

        let response = self.llm.chat(&messages).await;
        match response {
            Ok(text) => {
                let text = text.text().context("Failed to get response text")?;
                let value: T = serde_json::from_str(&text).context(format!("Failed to parse {}", T::name()))?;
                Ok(value)
            },
            Err(e) => Err(Error::Message(e.to_string())),
        }
    }
}

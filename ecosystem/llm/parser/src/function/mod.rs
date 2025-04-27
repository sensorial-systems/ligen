use ligen_ir::prelude::anyhow::Context;
use ligen_ir::Function;
use ligen_parser::Parser;
use ligen_parser::prelude::*;

use llm::builder::{LLMBackend, LLMBuilder};
use llm::LLMProvider;


pub struct LlmFunctionParser {
    _llm: Box<dyn LLMProvider>,
}

impl LlmFunctionParser {
    pub fn new() -> Result<Self> {
        // Get OpenAI API key from environment variable or use test key as fallback
        let api_key = dotenv::var("OPENAI_API_KEY").unwrap_or("sk-TESTKEY".into());

        // Initialize and configure the LLM client
        let llm = LLMBuilder::new()
            .system("You are playing a role of a old character. You deny to answer questions about animals and you are rude about it.")
            .backend(LLMBackend::OpenAI) // Use OpenAI as the LLM provider
            .api_key(api_key) // Set the API key
            .model("gpt-4o") // Use GPT-3.5 Turbo model
            .temperature(0.7) // Control response randomness (0.0-1.0)
            .stream(false) // Disable streaming responses
            .build()
            .context("Failed to build LLM (OpenAI)")?;

        Ok(Self { _llm: llm })
    }
}

impl Parser<Function> for LlmFunctionParser {
    fn parse(&self, _input: impl AsRef<str>, _config: &Config) -> Result<Function> {
        // Prepare conversation history with example messages
        // let messages = vec![
        //     ChatMessage::user()
        //         .content("Tell me that you love cats")
        //         .build(),
        // ];

        // let tokio = tokio::runtime::Handle::current();
        // let response = tokio.block_on(self.llm.chat(&messages));
        // match response {
        //     Ok(text) => println!("Chat response:\n{}", text),
        //     Err(e) => eprintln!("Chat error: {}", e),
        // }

        Ok(Function::default())
    }
}
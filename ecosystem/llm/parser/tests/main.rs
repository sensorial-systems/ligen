use ligen_ir::{function, module, Function, Module};
use ligen_llm_parser::GeneralLlmParser;
use ligen_transformer::{assert::async_assert_eq, prelude::*};

#[tokio::test]
async fn function() -> Result<()> {
    if std::env::var("GITHUB_ACTIONS").is_ok() {
        println!("Running in GitHub Actions - skipping test because I don't want to pay for the API calls :)");
        return Ok(());
    }

    async_assert_eq(GeneralLlmParser::<Function>::new()?, function::mock::function_input_output(), "pub fn test(a: i32, b: i32) -> i32 { a + b }".to_string()).await?;
    Ok(())
}

#[tokio::test]
async fn module() -> Result<()> {
    if std::env::var("GITHUB_ACTIONS").is_ok() {
        println!("Running in GitHub Actions - skipping test because I don't want to pay for the API calls :)");
        return Ok(());
    }

    async_assert_eq(GeneralLlmParser::<Module>::new()?, module::mock::module_types(), "pub mod types { pub struct Structure; pub enum Enumeration; }".to_string()).await?;
    Ok(())
}

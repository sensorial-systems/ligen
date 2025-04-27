use ligen_llm_parser::LlmFunctionParser;
use ligen_transformer::prelude::*;

#[tokio::test]
async fn main() -> Result<()> {
    let parser = LlmFunctionParser::new()?;

    let function = parser.parse("fn add(a: i32, b: i32) -> i32 { a + b }", &Config::default())?;
    println!("{:#?}", function);

    Ok(())
}
use ligen_llm_parser::LlmFunctionParser;
use ligen_parser::{Parser, ParserConfig};

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = LlmFunctionParser::new()?;

    let function = parser.parse("fn add(a: i32, b: i32) -> i32 { a + b }", &ParserConfig::default())?;
    println!("{:#?}", function);

    Ok(())
}
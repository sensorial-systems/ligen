use ligen_ir::Function;
use ligen_llm_parser::GeneralLlmParser;
use ligen_transformer::prelude::*;

#[tokio::test]
async fn main() -> Result<()> {
    if std::env::var("GITHUB_ACTIONS").is_ok() {
        println!("Running in GitHub Actions - skipping test because I don't want to pay for the API calls :)");
        return Ok(());
    }

    let parser = GeneralLlmParser::<Function>::new()?;

    match parser.parse("pub async fn add(a: i32, b: i32) -> i32 { a + b }", &Config::default()).await {
        Ok(function) => println!("{:#?}", function),
        Err(e) => println!("{}", e),
    }

    Ok(())
}
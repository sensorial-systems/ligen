use ligen_ir::{function, module, Function, Module};
use ligen_llm_parser::{discovery::{ProjectDiscovery, ProjectFiles, StructuredProjectFiles}, GeneralLlmParser};
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

#[tokio::test]
async fn project_discovery() -> Result<()> {
    if std::env::var("GITHUB_ACTIONS").is_ok() {
        println!("Running in GitHub Actions - skipping test because I don't want to pay for the API calls :)");
        return Ok(());
    }

    let project_directory = ProjectFiles::from_path("/home/notdanilo/dev/3rd/NotaFiscalSP")?;
    println!("{}", project_directory);
    let project_discovery = GeneralLlmParser::<ProjectDiscovery>::new()?;
    let project_discovery = project_discovery.parse(&project_directory.to_string(), &Config::default()).await?;
    println!("{:#?}", project_discovery);
    let structured_project_files = StructuredProjectFiles::new(&project_directory, &project_discovery);
    println!("{:#?}", structured_project_files);
    Ok(())
}

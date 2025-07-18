use ligen_ir::{function, module, Function, Module};
use ligen_llm_parser::{discovery::{ProjectDiscovery, ProjectFiles, StructuredProjectFiles}, GeneralLlmParser};
use ligen_transformer::{assert::async_assert_eq, prelude::*};

#[tokio::test]
async fn function() -> Result<()> {
    if let Ok(parser) = GeneralLlmParser::<Function>::new() {
        async_assert_eq(parser, function::mock::function_input_output(), "pub fn test(a: i32, b: i32) -> i32 { a + b }".to_string()).await?;
    } else {
        println!("Skipping test because I don't want to pay for the API calls :)");
    }

    Ok(())
}

#[tokio::test]
async fn module() -> Result<()> {
    if let Ok(parser) = GeneralLlmParser::<Module>::new() {
        async_assert_eq(parser, module::mock::module_types(), "pub mod types { pub struct Structure; pub enum Enumeration; }".to_string()).await?;
    } else {
        println!("Skipping test because I don't want to pay for the API calls :)");
    }

    Ok(())
}

#[tokio::test]
async fn project_discovery() -> Result<()> {
    if let Ok(parser) = GeneralLlmParser::<ProjectDiscovery>::new() {
        let project_directory = ProjectFiles::from_path("/home/notdanilo/dev/3rd/NotaFiscalSP")?; // FIXME: This is a hardcoded path.
        let project_discovery = parser.parse(&project_directory.to_string(), &Config::default()).await?;
        let structured_project_files = StructuredProjectFiles::new(&project_directory, &project_discovery);
        println!("{structured_project_files:#?}");
    } else {
        println!("Skipping test because I don't want to pay for the API calls :)");
    }

    Ok(())
}

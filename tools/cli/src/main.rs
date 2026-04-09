use std::path::{Path, PathBuf};
use clap::Parser as Clap;
use ligen_idl::prelude::*;
use ligen_idl::Registry;
use ligen_transformer::{Config, ConfigSet, Transformer};
use ligen_transformer::generator::Generator;

#[derive(Clap, Debug)]
pub struct Args {
    #[arg(short, long)]
    parser: String,

    #[arg(short, long)]
    generator: String,

    #[arg(short, long)]
    input: PathBuf,

    #[arg(short, long)]
    output: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // 1. Select Parser and Parse
    let registry = parse(&args.parser, &args.input)?;
    
    // 2. Select Generator and Generate
    generate(&args.generator, registry, &args.output)?;
    
    Ok(())
}

fn parse(parser_name: &str, input: &Path) -> Result<Registry> {
    let config = Config::default();
    match parser_name.to_lowercase().as_str() {
        "rust" => {
            let parser = ligen_rust_parser::RustRegistryParser::new();
            parser.transform(input, &config)
        }
        "python" => {
            let parser = ligen_python_parser::PythonParser::default();
            parser.transform(input, &config)
        }
        "openapi" => {
            let parser = ligen_openapi_parser::OpenAPILibraryParser::new();
            let library = parser.transform(input, &config)?;
            Ok(Registry { libraries: vec![library] })
        }
        "anchor" => {
            let parser = ligen_anchor_parser::library::LibraryParser::default();
            let library = parser.transform(input, &config)?;
            Ok(Registry { libraries: vec![library] })
        }
        _ => Err(Error::Message(format!("Parser not found: {}", parser_name)))
    }
}

fn generate(generator_name: &str, registry: Registry, output: &Path) -> Result<()> {
    let mut config = Config::default();
    config.set("ligen::output-dir", output.to_string_lossy().to_string());
    
    for library in registry.libraries {
        match generator_name.to_lowercase().as_str() {
            "rust-client" => {
                let generator = ligen_rust_client_generator::RustClientGenerator::default();
                generator.generate(&library, &config)?;
            }
            "pyo3" => {
                let generator = ligen_rust_pyo3_importer::LibraryGenerator::default();
                generator.generate(&library, &config)?;
            }
            "anchor" => {
                let generator = ligen_anchor_generator::AnchorGenerator::new();
                let idl = generator.generate(&library, &config)?;
                let file_path = output.join(format!("{}.json", library.identifier));
                let json = serde_json::to_string_pretty(&idl)?;
                std::fs::write(file_path, json)?;
            }
            _ => return Err(Error::Message(format!("Generator not found: {}", generator_name)))
        }
    }
    Ok(())
}

use std::path::PathBuf;

use clap::Parser;
use ligen_generator::Generator;
use ligen_ir::prelude::*;
use ligen_parser::Parser as LigenParser;
use ligen_python_parser::{PythonParser, PythonParserConfig};
use ligen_rust_pyo3_importer::LibraryGenerator;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    parser: String,

    #[arg(short, long)]
    generator: String,

    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: String
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("{:#?}", args);
    let parser = PythonParser::default();
    let mut config = PythonParserConfig::default();
    config.set_class_variables_as_properties(true);
    let input = PathBuf::from(&args.input);
    let library = parser.parse(input.as_path(), &config)?;
    LibraryGenerator::default().generate(&library, PathBuf::from(&args.output).as_path())?;
    Ok(())
}

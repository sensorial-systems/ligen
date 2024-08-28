use std::path::PathBuf;

use clap::Parser;
use ligen_generator::Generator;
use ligen_ir::{prelude::*, Registry};
use ligen_parser::{Parser as LigenParser, ParserConfig};
use ligen_python_parser::{PythonParser, PythonParserConfig};
use ligen_rust_pyo3_importer::LibraryGenerator;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    parser: String,

    #[arg(short, long)]
    generator: String,

    #[arg(short, long)]
    input: PathBuf,

    #[arg(short, long)]
    output: PathBuf
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("{:#?}", args);
    let (parser, config): (Box<dyn LigenParser<&std::path::Path, Output = Registry>>, ParserConfig) = if args.parser.to_lowercase() == "python" {
        let parser = PythonParser::default();
        let config = PythonParserConfig::default();
        (Box::new(parser), config.into())
    } else {
        panic!("Parser not found.");
    };
    let registry = parser.parse(args.input.as_path(), &config)?;
    for library in registry.libraries.iter() {
        LibraryGenerator::default().generate(&library, PathBuf::from(&args.output).as_path())?;
    }
    Ok(())
}

use std::path::PathBuf;

use clap::Parser as Clap;
use ligen_ir::{prelude::*, Registry};
use ligen_parser::Parser;
use ligen_python_parser::PythonParser;
use ligen_rust_parser::RustParser;

#[derive(Clap, Debug)]
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
    let parser: Box<dyn Parser<&std::path::Path, Output = Registry>> = if args.parser.to_lowercase() == "python" {
        Box::new(PythonParser::default())
    } else if args.parser.to_lowercase() == "rust" {
        Box::new(RustParser::default())
    } else {
        panic!("Parser not found.");
    };
    let config = parser.config();
    let _registry = parser.parse(args.input.as_path(), &config)?;
    // for library in registry.libraries.iter() {
    //     LibraryGenerator::default().generate(&library, PathBuf::from(&args.output).as_path())?;
    // }
    Ok(())
}

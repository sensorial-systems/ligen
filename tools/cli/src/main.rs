use std::path::PathBuf;

use clap::Parser;
use ligen_generator::Generator;
use ligen_ir::prelude::*;
use ligen_parser::{Parser as LigenParser, ParserConfigSet};
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
    config.set("ligen::python::as-opaque::HttpUrl", true);
    config.set("ligen::python::as-opaque::FilePath", true);
    config.set("ligen::python::as-opaque::bytes", true);
    config.set("ligen::python::as-opaque::Path", true);
    config.set("ligen::python::as-opaque::SETTING_VALUE", true);
    config.set("ligen::python::as-opaque::MUTE_ALL", true);
    config.set("ligen::python::as-opaque::DATA_ORDERING", true);
    config.set("ligen::python::as-opaque::TIME_FRAME", true);
    config.set("ligen::python::as-opaque::POST_TYPE", true);
    config.set("ligen::python::as-opaque::SEARCH_MODE", true);
    config.set("ligen::python::as-opaque::SEND_ATTRIBUTES_MEDIA", true);
    config.set("ligen::python::as-opaque::SEND_ATTRIBUTE", true);
    config.set("ligen::python::as-opaque::SELECTED_FILTER", true);
    config.set("ligen::python::as-opaque::BOX", true);
    config.set("ligen::python::as-opaque::TIMELINE_FEED_REASON", true);
    config.set("ligen::python::as-opaque::REELS_TRAY_REASON", true);
    
    let input = PathBuf::from(&args.input);
    let library = parser.parse(input.as_path(), &config)?;
    LibraryGenerator::default().generate(&library, PathBuf::from(&args.output).as_path())?;
    Ok(())
}

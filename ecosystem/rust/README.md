# Ligen Rust Ecosystem

This module provides parsing and generation tools for Rust code in Ligen.

### Features
- Parse Rust crates into Ligen IR using `ligen-rust-parser`.
- Generate Rust bindings with PyO3 integration via `ligen-rust-pyo3-importer`.
- Example exporter and generator in subcrates.

### Usage
In your project:
```
use ligen_rust_parser::RustParser;
use ligen::idl::Library;

let library = RustParser::parse("path/to/rust/project")?;
```

For generation, see the main Ligen docs.

### TODOs and Limitations
- Incomplete support for unions, inherited dependencies.
- Expand with more examples.
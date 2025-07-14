# Ligen Editor

Ligen Editor is a Rust-based code editor designed specifically for working with Ligen IR. It provides a simple and intuitive interface for:

* Parsing libraries to Ligen IR
* Saving and loading IRs
* Getting metrics of the parsed IR
* Generating language bindings

Everything that the Editor does can also be done programmatically by using the Ligen library.

### Installation
Install via Cargo:
```
cargo install ligen-editor
```

### Running Ligen Editor
```
ligen-editor
```

Alternatively, build from source:
```
git clone https://github.com/sensorial-systems/ligen
cd ligen
cargo run -p ligen-editor
```

### Usage
- Load a project directory to parse into IR.
- View metrics and generate bindings for supported languages.
- Save IR for later use.
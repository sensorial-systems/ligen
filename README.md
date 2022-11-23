# Ligen
Ligen (Language Interface Generator) is an extensible macro-based multi-language binding
generator.

### Features

* Zero-effort binding generation
* ...

### Running the examples

1. Generate the bindings: `ligen $ cargo build --features=bindings`
2. Running the C example:
    1. Generate build directory: `ligen/examples/example-c $ mkdir build`
    2. Generate project files: `ligen/examples/example-c/build $ cmake ..`
    3. Build: `ligen/examples/example-c/build $ cmake --build .`
    4. Run: `ligen/examples/example-c/build/Debug/example-c`

### Getting started

Here are a few links to get started:
* [List of officially supported languages](https://github.com/sensorial-systems/ligen/tree/dev/generators).
* [Usage example](https://github.com/sensorial-systems/ligen/tree/dev/examples/example).

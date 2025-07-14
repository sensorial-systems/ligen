[![](https://dcbadge.vercel.app/api/server/rzaesS82MT)](https://discord.gg/rzaesS82MT)

# Ligen

Ligen (Language Interface Generator - IPA: /ˈliɡən/) is an extensible multi-language binding generator designed to create seamless interfaces between different programming languages with minimal effort.

### Features

* **Zero-effort binding generation**: Automatically parse code from one language and generate bindings for others using an Intermediate Representation (IR).
* **Multi-language support**: Currently includes parsers and generators for Python, Rust, C/C++, Anchor (Solana), WGSL, and experimental LLM-based parsing.
* **Extensible ecosystem**: Easily add support for new languages via modular parsers and generators.
* **Tools included**: CLI for binding generation and a GUI editor for working with IR (parsing, metrics, and generation).
* **LLM integration**: Uses AI (e.g., OpenAI) for semantic code discovery and parsing.

### Getting Started

#### Prerequisites
- Rust and Cargo (for building and running).
- Optional: OpenAI API key for LLM features (set as `OPENAI_API_KEY` in your environment).

#### Installation
1. Clone the repository:
   ```
   git clone https://github.com/sensorial-systems/ligen
   cd ligen
   ```
2. Build the workspace:
   ```
   cargo build
   ```

#### Using the CLI
Parse a project and generate bindings:
```
cargo run -p ligen-cli -- --parser rust --generator rust --input path/to/project --output path/to/output
```

#### Using the Editor
Run the GUI editor:
```
cargo run -p ligen-editor
```
(Once published: `cargo install ligen-editor`)

For more details, see [Ecosystem README](./ecosystem/README.md) and [Tools README](./tools/README.md).

### Contributing
We welcome contributions! See CONTRIBUTING.md for guidelines. Focus areas include completing language ecosystems, fixing TODOs, and adding tests.

### License
Apache-2.0 - see [LICENSE](./LICENSE) for details.

[![](https://dcbadge.vercel.app/api/server/rzaesS82MT)](https://discord.gg/rzaesS82MT)

# Ligen

Ligen (Language Interface Generator - IPA: /ˈliɡən/) is an extensible multi-language binding generator designed to create seamless interfaces between different programming languages with minimal effort.

### Features

#### Language & Framework Support

| Ecosystem | Parser | Generator | Exporter | Notes |
| :--- | :---: | :---: | :---: | :--- |
| **Rust** | ✅ | ✅ | ✅ | Full support, foundational ecosystem |
| **C / C++** | - | ✅ | - | Focused on FFI and CMake integration |
| **Python** | ✅ | - | - | Leveraging PyO3 for parsing |
| **Anchor (Solana)** | ✅ | ✅ | - | Blockchain IDL support |
| **OpenAPI** | ✅ | - | - | Parsing specifications for client gen |
| **WGSL** | - | ✅ | - | WebGPU Shading Language |
| **LLM** | ✅ | ✅ | - | Experimental AI-driven parsing |

#### Tools

| Tool | Description | Status |
| :--- | :--- | :--- |
| **CLI** | Command-line interface for binding generation | ✅ Stable |
| **Editor** | GUI for working with IR, metrics, and generation | 🛠️ In Development |
| **MCP Server** | Model Context Protocol for AI agent integration | ✅ Stable |


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

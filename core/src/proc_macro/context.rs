//! Context about the proc-macro execution.

use std::path::PathBuf;
use serde::{Serialize, Deserialize};

/// Release or Debug.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum BuildType {
    #[allow(missing_docs)]
    Release,
    #[allow(missing_docs)]
    Debug
}

impl ToString for BuildType {
    fn to_string(&self) -> String {
        format!("{:#?}", self)
    }
}

/// Arguments passed from `cargo-ligen`.
#[derive(Debug, Serialize, Deserialize)]
pub struct Arguments {
    /// The build type.
    pub build_type: BuildType,
    /// The build target directory.
    pub target_dir: PathBuf,
    /// The Cargo.toml manifest path.
    pub manifest_path: PathBuf,
    /// The Cargo.toml workspace manifest, if any.
    pub workspace_path: Option<PathBuf>
}

impl Arguments {
    /// Generates a JSON representation of Arguments in CARGO_LIGEN_ARGUMENTS.
    pub fn to_env(&self) {
        let json = serde_json::to_string(self).expect("Couldn't serialize.");
        std::env::set_var("CARGO_LIGEN_ARGUMENTS", json);
    }

    /// Parses the JSON representation from CARGO_LIGEN_ARGUMENTS.
    pub fn from_env() -> Self {
        let json_string = std::env::var("CARGO_LIGEN_ARGUMENTS").expect("Couldn't set the environment variables.");
        serde_json::from_str(&json_string).expect("Couldn't parse JSON string.")
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// Context struct.
pub struct Context {
    /// The current SourceFile.
    pub source_file: SourceFile,
    /// Arguments.
    pub arguments: Arguments
}

#[derive(Debug, Serialize, Deserialize)]
/// SourceFile struct.
pub struct SourceFile {
    /// If it's a real file.
    pub is_real: bool,
    /// The source file path.
    pub path: PathBuf,
}

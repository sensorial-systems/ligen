//! Arguments definition module.

use crate::prelude::*;
use crate::generator::context::BuildType;

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Arguments passed from `cargo-ligen`.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Arguments {
    /// The name of the crate
    pub crate_name: String,
    /// The build type.
    pub build_type: BuildType,
    /// The build target directory passed with `--target-dir`.
    pub target_dir: PathBuf,
    /// The Cargo.toml manifest path passed with `--manifest-path`.
    pub manifest_path: PathBuf,
    /// The Cargo.toml workspace manifest.
    pub workspace_path: Option<PathBuf>,
    /// Workspace member to build passed with `--package` or `-p`.
    pub workspace_member_package_id: Option<String>,
}

impl Arguments {
    /// Generates a JSON representation of Arguments in CARGO_LIGEN_ARGUMENTS.
    pub fn to_env(&self) -> Result<()> {
        let json = serde_json::to_string(self)?;
        Ok(std::env::set_var("CARGO_LIGEN_ARGUMENTS", json))
    }

    /// Parses the JSON representation from CARGO_LIGEN_ARGUMENTS.
    pub fn from_env() -> Result<Self> {
        let json_string = std::env::var("CARGO_LIGEN_ARGUMENTS")?;
        let mut arguments: Self = serde_json::from_str(&json_string)?;
        arguments.manifest_path = Path::new(&std::env::var("CARGO_MANIFEST_DIR")?).to_path_buf();
        Ok(arguments)
    }
}
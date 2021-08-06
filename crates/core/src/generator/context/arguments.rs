//! Arguments definition module.

use crate::prelude::*;
use crate::generator::context::BuildType;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Arguments passed from `cargo-ligen`.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Arguments {
    /// The name of the crate
    pub crate_name: String,
    /// The build type.
    pub build_type: BuildType,
    /// The build target directory.
    pub target_dir: PathBuf,
    /// The Cargo.toml manifest path passed with `--target-dir`.
    pub manifest_path: PathBuf,
    /// The Cargo.toml workspace manifest passed with `--manifest-path`.
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
        Ok(serde_json::from_str(&json_string)?)
    }
}
use ligen_ir::prelude::*;
use super::CargoBuilder;
use std::path::PathBuf;
use ligen_utils::conventions::naming::NamingConvention;
use std::ffi::OsString;
use ligen_ir::ProjectInfo;
use ligen_rust::prelude::LigenProjectInfo;
use ligen_traits::build::BuildSystem;

/// Cargo project.
pub struct CargoProject {
    pub path: PathBuf,
    pub name: NamingConvention,
    pub manifest_path: PathBuf,
}

impl CargoProject {
    /// Read the current project AST.
    pub fn current() -> Result<Self> {
        let path = std::env::current_dir()?;
        Self::try_from(path.as_path())
    }
}

impl TryFrom<&std::path::Path> for CargoProject {
    type Error = Error;
    fn try_from(path: &std::path::Path) -> Result<Self> {
        CargoBuilder::check_build()?;

        let path = if path.file_name() == Some(&OsString::from("Cargo.toml")) {
            path.parent().expect("Failed to get Cargo.toml's parent.")
        } else {
            path
        }.to_path_buf();

        let manifest_path = path.join("Cargo.toml");
        let manifest = cargo_toml::Manifest::from_path(manifest_path.as_path())?;
        let package = manifest.package.ok_or_else(|| Error::Message("Package not found in Cargo.toml.".into()))?;
        let crate_name = package.name;
        let name = NamingConvention::try_from(crate_name.as_str())?;
        Ok(Self { path, name, manifest_path })
    }
}

impl TryFrom<CargoProject> for Project {
    type Error = Error;
    fn try_from(from: CargoProject) -> Result<Self> {
        let name = from.name;
        let directory = from.path;
        let manifest_path = from.manifest_path;
        let project = ProjectInfo { name: name.clone(), directory: directory.clone() };
        let root_module = LigenProjectInfo(project).try_into()?; // FIXME: Using LigenProjectInfo here is weird. All the types prefixed with Ligen should be private in ligen-rust.
        Ok(Self { name, directory, manifest_path, root_module })
    }
}

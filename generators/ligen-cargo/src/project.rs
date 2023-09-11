use ligen_ir::prelude::*;
use super::CargoBuilder;
use std::path::PathBuf;
use ligen_ir::conventions::naming::NamingConvention;
use std::ffi::OsString;
use ligen_parsing::{Context, ParseFrom};
use ligen_rust::parser::project::RustProject;
use ligen_traits::build::BuildSystem;
use ligen_utils::transformers::alias::ReplaceCrateAlias;
use ligen_utils::transformers::path::RelativePathToAbsolutePath;
use ligen_utils::transformers::Transformable;

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
        let manifest = cargo_toml::Manifest::from_path(manifest_path.as_path()).map_err(|e| Error::Generic(Box::new(e)))?;
        let package = manifest.package.ok_or_else(|| Error::Message("Package not found in Cargo.toml.".into()))?;
        let crate_name = package.name;
        let name = NamingConvention::try_from(crate_name.as_str())?;
        Ok(Self { path, name, manifest_path })
    }
}

impl ParseFrom<CargoProject> for Project {
    fn parse(context: &Context<'_>, from: CargoProject) -> Result<Self> where Self: Sized {
        let project = RustProject::try_from(from.path)?;
        let project = Project::parse(context,project)?;
        // FIXME: Move this to a more generic place.
        let project = project.transforms(&[&ReplaceCrateAlias, &RelativePathToAbsolutePath]);
        Ok(project)
    }
}

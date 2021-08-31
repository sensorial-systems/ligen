//! Project representation.

use crate::generator::TemporaryFFIProject;
use crate::ir::Module;
use crate::prelude::*;
use crate::conventions::naming::NamingConvention;
use std::path::PathBuf;
use std::ffi::OsString;


/// Project representation.
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct Project {
    path: PathBuf,
    name: NamingConvention,
    manifest_path: PathBuf,
    target_dir: PathBuf,
    pub root_module: Module,
}

impl Project {
    /// Project path.
    pub fn path(&self) -> &std::path::Path {
        self.path.as_path()
    }
    /// Get manifest path.
    pub fn manifest_path(&self) -> PathBuf {
        self.manifest_path.clone()
    }

    /// Get the target dir.
    pub fn target_dir(&self) -> PathBuf {
        self.target_dir.clone()
    }

    /// Get the project name.
    pub fn name(&self) -> &NamingConvention {
        &self.name
    }

    fn check_build() -> Result<()> {
        // The project isn't available if we are currently building the TemporaryProject.
        if TemporaryFFIProject::is_building() {
            Err(Error::Message("Use the following snippet and ignore errors: if let Ok(project) = Project::read() { todo!(\"Your code here.\") }.".into()))
        } else {
            Ok(())
        }
    }

    /// Read the current project AST.
    pub fn current() -> Result<Self> {
        let path = std::env::current_dir()?;
        Self::try_from(path.as_path())
    }
}

impl TryFrom<&std::path::Path> for Project {
    type Error = Error;
    fn try_from(path: &std::path::Path) -> Result<Self> {
        Self::check_build()?;

        let path = if path.file_name() == Some(&OsString::from("Cargo.toml")) {
            path.parent().expect("Failed to get Cargo.toml's parent.")
        } else {
            path
        }.to_path_buf();

        let root_module = path
            .join("src")
            .join("lib.rs");

        let manifest_path = path.join("Cargo.toml");
        let manifest  = cargo_toml::Manifest::from_path(manifest_path.as_path())?;
        let package = manifest.package.ok_or_else(|| Error::Message("Package not found in Cargo.toml.".into()))?;
        let crate_name = package.name;
        let name = NamingConvention::try_from(crate_name.as_str())?;
        let root_module = Module::try_from(root_module.as_path())?;
        let target_dir = path.join("target");
        Ok(Self { path, name, root_module, target_dir, manifest_path })
    }
}

#[cfg(test)]
mod tests {
    use crate::ir::Project;
    use std::convert::TryFrom;
    use std::path::PathBuf;

    fn project_directory() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("test-project")
    }

    #[test]
    fn project_from_cargo() {
        project(project_directory().join("Cargo.toml"));
    }

    #[test]
    fn project_from_directory() {
        project(project_directory());
    }

    fn project(path: PathBuf) {
        let project = Project::try_from(path.as_path()).expect("Failed to get the project from the specified path.");
        assert_eq!(project.name().to_string(), "test-project");
    }
}
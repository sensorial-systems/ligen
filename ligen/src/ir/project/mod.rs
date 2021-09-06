//! Project representation.

use crate::generator::TemporaryFFIProject;
use crate::ir::Module;
use crate::prelude::*;
use crate::conventions::naming::NamingConvention;
use std::path::{PathBuf, Path};
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
    pub fn path(&self) -> &Path {
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
        // The TemporaryProject shouldn't be available if we are currently building it.
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

impl TryFrom<&Path> for Project {
    type Error = Error;
    fn try_from(path: &Path) -> Result<Self> {
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
        let manifest = cargo_toml::Manifest::from_path(manifest_path.as_path())?;
        let package = manifest.package.ok_or_else(|| Error::Message("Package not found in Cargo.toml.".into()))?;
        let crate_name = package.name;
        let name = NamingConvention::try_from(crate_name.as_str())?;
        let root_module = Module::try_from(root_module.as_path())?;
        let target_dir = target_dir_from_out_dir(None)?;
        Ok(Self { path, name, root_module, target_dir, manifest_path })
    }
}

fn target_dir_from_out_dir(out_dir: Option<String>) -> Result<PathBuf> {
    let out_dir = if let Some(out_dir) = out_dir {
        out_dir
    } else {
        std::env::var("OUT_DIR")?
    };
    let path = Path::new(&out_dir);
    if let Some(ancestor) = path.ancestors().collect::<Vec<_>>().get(4) {
        Ok(ancestor.to_path_buf())
    } else {
        Err(Error::Message("OUT_DIR isn't in the expected format.".into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_dir() {
        let path = target_dir_from_out_dir(Some("target/debug/build/counter-cb2a7557d006cbbc/out".into())).expect("Failed to get target dir.");
        assert_eq!(Path::new("target"), path.as_path());
    }
}
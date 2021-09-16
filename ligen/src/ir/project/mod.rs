//! Project representation.

use crate::generator::{TemporaryFFIProject, ModuleVisitor, ProjectVisitor};
use crate::ir::Module;
use crate::prelude::*;
use crate::conventions::naming::NamingConvention;
use std::path::PathBuf;
use std::ffi::OsString;


/// Project representation.
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct Project {
    pub path: PathBuf,
    pub name: NamingConvention,
    pub manifest_path: PathBuf,
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

    /// Constructs the project visitor.
    pub fn visitor(&self) -> ProjectVisitor {
        ProjectVisitor::new((), self.clone())
    }

    /// Constructs the root module visitor.
    pub fn root_module_visitor(&self) -> ModuleVisitor {
        let project_visitor = self.visitor();
        (&project_visitor.child(self.root_module.clone())).into()
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
        let manifest = cargo_toml::Manifest::from_path(manifest_path.as_path())?;
        let package = manifest.package.ok_or_else(|| Error::Message("Package not found in Cargo.toml.".into()))?;
        let crate_name = package.name;
        let name = NamingConvention::try_from(crate_name.as_str())?;
        let mut root_module = Module::try_from(root_module.as_path())?;
        // TODO: Use SnakeCase::from(name.clone()).into() instead?
        root_module.name = "crate".into();
        Ok(Self { path, name, root_module, manifest_path })
    }
}

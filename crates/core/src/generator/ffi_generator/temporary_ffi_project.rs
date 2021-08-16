//! Temporary project to build the externalized FFI functions.

use crate::prelude::*;
use crate::generator::{File, BuildType};
use tempfile::TempDir;
use std::path::Path;

/// Temporary project to build the externalized FFI functions.
#[derive(Debug)]
#[allow(missing_docs)]
pub struct TemporaryFFIProject {
    temporary_directory: TempDir,
    name: String,
    pub cargo_file: File,
    pub lib_file: File
}

impl TemporaryFFIProject {
    /// Creates a new temporary project which depends on `dependency_path`.
    pub fn new<S, P>(name: S, dependency_path: P) -> Result<Self>
    where S: AsRef<str>,
          P: AsRef<Path>
    {
        let temporary_directory = tempfile::tempdir()?;
        let temporary_path = temporary_directory.path();
        let lib_file = File::new(temporary_path.join("src").join("lib.rs"), String::new());
        let cargo_content = TemporaryFFIProject::content_template(&name, "0.1.0", dependency_path);
        let cargo_file = File::new(temporary_path.join("Cargo.toml"), cargo_content);
        let name = name.as_ref().into();
        Ok(Self { name, temporary_directory, cargo_file, lib_file })
    }

    fn content_template<A, B, P>(name: A, version: B, dependency_path: P) -> String
        where A: AsRef<str>,
              B: AsRef<str>,
              P: AsRef<Path>
    {
        let name = name.as_ref();
        let version = version.as_ref();
        let path = dependency_path.as_ref().display();
        format!(include_str!("Cargo.template.toml"), name = name, version = version, path = path)
    }

    /// Saves the project files.
    pub fn save_files(&self) -> Result<()> {
        self.lib_file.save()?;
        self.cargo_file.save()
    }

    /// Builds the project.
    pub fn build(&self, build_type: BuildType) -> Result<()> {
        std::env::set_var("IS_BUILDING", "YES");
        let mut build_command = std::process::Command::new("cargo");
        let mut build_command = build_command.arg("build");
        if let BuildType::Release = build_type {
            build_command = build_command.arg("--release");
        }
        build_command
            .arg("--manifest-path")
            .arg(self.cargo_file.path.display().to_string())
            .arg("--target-dir")
            .arg(self.temporary_directory.path().join("target").display().to_string())
            .status()?;
        Ok(())
    }

    /// Check if the temporary project is currently building.
    pub fn is_building() -> bool {
        std::env::var("IS_BUILDING").unwrap_or("NO".into()) == "YES"
    }

    fn to_library_name_convention<S: AsRef<str>>(crate_name: S) -> String {
        let name = crate_name.as_ref();

        #[cfg(target_family = "windows")]
        let name = format!("{}.lib", name);

        #[cfg(target_family = "unix")]
        let name = format!("lib{}.a", name);

        name
    }
    /// Copy the generated static library to ligen's repository.
    pub fn transfer_static_library_to_ligen<P: AsRef<Path>>(&self, target_dir: P, build_type: BuildType) -> Result<()> {
        let file_name = Self::to_library_name_convention(format!("ffi_{}", self.name));
        let target_file_name = Self::to_library_name_convention(&self.name);

        let from_path = self
            .temporary_directory
            .path()
            .join("target")
            .join(build_type.to_string().to_lowercase())
            .join(file_name);

        let to_path = target_dir
            .as_ref()
            .join("ligen")
            .join(&self.name)
            .join("lib")
            .join(target_file_name);

        crate::utils::fs::copy(&from_path, &to_path)
    }
}

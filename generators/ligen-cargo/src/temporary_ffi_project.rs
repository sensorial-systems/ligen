//! Temporary project to build the externalized FFI functions.
// FIXME: Separate into CargoProject and CargoBuilder.
// TODO: Speed up build process by sharing the same target folder.

use crate::prelude::*;
use crate::generator::File;
use std::path::{Path, PathBuf};
use ligen_utils::conventions::naming::{NamingConvention, SnakeCase};
use ligen_traits::build::BuildProfile;

/// Temporary project to build the externalized FFI functions.
#[derive(Debug)]
#[allow(missing_docs)]
pub struct TemporaryFFIProject {
    name: String,
    path: PathBuf,
    pub cargo_file: File,
    pub lib_file: File
}

impl TemporaryFFIProject {
    /// Creates a new temporary project which depends on `dependency_path`.
    pub fn new<S, P>(name: S, dependency_path: P) -> Result<Self>
    where S: AsRef<str>,
          P: AsRef<Path>
    {
        // let temporary_directory = tempfile::tempdir()?;
        // let temporary_path = temporary_directory.path();
        let path = std::env::current_dir()?.join("target/ligen/ffi/").join(name.as_ref());
        std::fs::create_dir_all(&path)?;
        let lib_file = File::new(path.join("src").join("lib.rs"), String::new());
        let cargo_content = TemporaryFFIProject::content_template(&name, "0.1.0", dependency_path);
        let cargo_file = File::new(path.join("Cargo.toml"), cargo_content);
        let name = name.as_ref().into();
        Ok(Self { name, path, cargo_file, lib_file })
    }

    fn content_template<A, B, P>(name: A, version: B, dependency_path: P) -> String
        where A: AsRef<str>,
              B: AsRef<str>,
              P: AsRef<Path>
    {
        let name = name.as_ref();
        let version = version.as_ref();
        let path = dependency_path.as_ref().display().to_string().replace("\\", "/");
        format!(include_str!("Cargo.template.toml"), name = name, version = version, path = path)
    }

    /// Saves the project files.
    pub fn save_files(&self) -> Result<()> {
        self.lib_file.save()?;
        self.cargo_file.save()
    }

    fn to_dynamic_library_name_convention<S: AsRef<str>>(crate_name: S) -> String {
        let name = crate_name.as_ref();

        #[cfg(target_family = "windows")]
            let name = format!("{}.dll", name);

        #[cfg(target_family = "unix")]
            let name = format!("lib{}.so", name);

        name
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
    pub fn transfer_libraries_to_ligen<P: AsRef<Path>>(&self, target_dir: P, build_profile: BuildProfile) -> Result<()> {
        let name = NamingConvention::try_from(self.name.as_str())?;
        let name = SnakeCase::from(name).to_string();
        let file_name = Self::to_library_name_convention(format!("ffi_{}", name));
        let target_file_name = Self::to_library_name_convention(&name);

        let from_path = self
            .path
            .join("target")
            .join(build_profile.to_string().to_lowercase())
            .join(file_name);

        let to_path = target_dir
            .as_ref()
            .join("ligen")
            .join(&self.name)
            .join("lib")
            .join(target_file_name);

        crate::utils::fs::copy(&from_path, &to_path)?;

        let name = NamingConvention::try_from(self.name.as_str())?;
        let name = SnakeCase::from(name).to_string();
        let file_name = Self::to_dynamic_library_name_convention(format!("ffi_{}", name));
        let target_file_name = Self::to_dynamic_library_name_convention(&name);

        let from_path = self
            .path
            .join("target")
            .join(build_profile.to_string().to_lowercase())
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
use std::path::{PathBuf, Path};
use ligen_ir::prelude::*;
use ligen_traits::build::{BuildSystem, BuildProfile};
use ligen_utils::conventions::naming::SnakeCase;

/// Cargo builder.
#[derive(Clone, Copy, Debug)]
pub struct CargoBuilder;

impl CargoBuilder {
    /// Get target directory.
    pub fn target_dir() -> Result<PathBuf> {
        Self::target_dir_from_out_dir(None)
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
}

impl BuildSystem for CargoBuilder {
    fn check_build() -> Result<()> {
        // The TemporaryProject shouldn't be available if we are currently building it.
        // FIXME: This might cause problems when we generate multiple projects in a workspace.
        let is_building = std::env::var("LIGEN_IS_BUILDING").unwrap_or("NO".into()) == "YES";
        if is_building {
            Err(Error::Message("Use the following snippet and ignore errors: if let Ok(project) = Project::read() { todo!(\"Your code here.\") }.".into()))
        } else {
            Ok(())
        }
    }

    fn build_with_profile(&self, project: &Project, profile: BuildProfile) -> Result<()> {
        Self::check_build()?;
        std::env::set_var("LIGEN_IS_BUILDING", "YES");
        let mut build_command = std::process::Command::new("cargo");
        let mut build_command = build_command.arg("build");
        if let BuildProfile::Release = profile {
            build_command = build_command.arg("--release");
        }

        let project_name = SnakeCase::from(project.name.clone()).to_string();
        let ligen_path = Self::target_dir()
            .unwrap()
            .join("ligen");
        let project_path = ligen_path
            .join("rust")
            .join(&project_name);
        let manifest_path = project_path.join("Cargo.toml");
        let target_dir = project_path.join("target");

        let status = build_command
            .arg("--manifest-path")
            .arg(manifest_path)
            .arg("--target-dir")
            .arg(&target_dir)
            .status()?;
        if let Some(0) = status.code() {
            let profile = match profile {
                BuildProfile::Release => "release",
                BuildProfile::Debug => "debug"
            };
            let directory = std::fs::read_dir(target_dir.join(profile))?
                .filter_map(|entry| entry.ok())
                .map(|entry| entry);
            let libraries_dir = ligen_path
                .join("libraries")
                .join(&project_name);
            std::fs::create_dir_all(&libraries_dir)?;
            for entry in directory {
                if let Some(file_name) = entry.file_name().to_str() {
                    if entry.file_type()?.is_file() && file_name.contains(&project_name) {
                        std::fs::copy(&entry.path(), libraries_dir.join(file_name))?;
                    }
                }
            }
            Ok(())
        } else {
            Err(Error::Message("Failed to build the FFI (Foreign Function Interface) library.".into()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_dir() {
        let path = CargoBuilder::target_dir_from_out_dir(Some("target/debug/build/project-cb2a7557d006cbbc/out".into())).expect("Failed to get target dir.");
        assert_eq!(Path::new("target"), path.as_path());
    }
}
use std::path::{PathBuf, Path};
use ligen_ir::prelude::*;
use ligen_traits::build::{BuildSystem, BuildProfile};

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
        let is_building = std::env::var("IS_BUILDING").unwrap_or("NO".into()) == "YES";
        if is_building {
            Err(Error::Message("Use the following snippet and ignore errors: if let Ok(project) = Project::read() { todo!(\"Your code here.\") }.".into()))
        } else {
            Ok(())
        }
    }

    fn build(&self, project: &Project, profile: BuildProfile) -> Result<()> {
        Self::check_build()?;
        std::env::set_var("IS_BUILDING", "YES");
        let mut build_command = std::process::Command::new("cargo");
        let mut build_command = build_command.arg("build");
        if let BuildProfile::Release = profile {
            build_command = build_command.arg("--release");
        }
        let status = build_command
            .arg("--manifest-path")
            .arg(project.manifest_path.display().to_string())
            .arg("--target-dir")
            .arg(project.path.join("target").display().to_string())
            .status()?;
        if let Some(0) = status.code() {
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
        let path = CargoBuilder::target_dir_from_out_dir(Some("target/debug/build/example-cb2a7557d006cbbc/out".into())).expect("Failed to get target dir.");
        assert_eq!(Path::new("target"), path.as_path());
    }
}
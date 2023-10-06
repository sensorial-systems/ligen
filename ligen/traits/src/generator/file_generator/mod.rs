//! File generator module.

mod file;
pub mod template_based;

pub use file::*;
pub use template_based::*;

use crate::prelude::*;
use crate::generator::Generator;

use ligen_ir::conventions::naming::SnakeCase;
use ligen_utils::fs::write_file;
use std::path::{Path, PathBuf};

/// File generator.
pub trait FileGenerator {
    // TODO: Fetch this from the generator configuration instead and possibly default to something if it doesn't exist.
    /// Generation base path.
    fn base_path(&self) -> PathBuf;

    /// Generate files.
    fn generate_files(&self, project: &Project, file_set: &mut FileSet) -> Result<()>;

    /// Saves the file set.
    fn save_file_set(&self, project: &Project, file_set: FileSet) -> Result<()> {
        let target = std::env::var("OUT_DIR")
            .ok()
            .map(PathBuf::from)
            .and_then(|path| path
                .ancestors()
                .nth(4)
                .map(Path::to_path_buf))
            .unwrap_or(std::env::current_dir()?);
        let target_ligen_dir = target
            .join("ligen")
            .join(self.base_path());
        let project_dir = target_ligen_dir.join(SnakeCase::try_from(project.name().clone())?.to_string());
        for (_path, file) in file_set.files {
            let file_path = project_dir.join(file.path);
            write_file(&file_path, &file.content)?;
        }
        Ok(())
    }
}

impl <T: FileGenerator> Generator for T {
    fn generate(&self, project: &Project) -> Result<()> {
        let mut file_set = FileSet::default();
        self.generate_files(project, &mut file_set)?;
        self.save_file_set(project, file_set)?;
        Ok(())
    }
}
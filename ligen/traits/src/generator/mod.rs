//! Generators.

pub use file::*;
pub use file_generator::*;
pub use visitor::*;

use crate::prelude::*;
use ligen_utils::fs::write_file;

mod file;
use ligen_ir::visitor;
use std::path::{Path, PathBuf};

mod file_generator;
pub mod file_processor_visitor;

/// Generator trait.
pub trait Generator: FileGenerator {
    // TODO: Fetch this from the generator configuration instead and possibly default to something if it doesn't exist.
    /// Generation base path.
    fn base_path(&self) -> PathBuf;

    /// Main function called in the procedural proc_macro.
    fn generate(&self, root: &Project) -> Result<()> {
        // TODO: Is it still necessary? It isn`t used in separating-ligen-ir which is most recent.
        // let root = self.pre_process(root);
        let mut file_set = FileSet::default();
        let visitor = Visitor::new((),root.clone());
        self.generate_files(&mut file_set, &visitor)?;
        self.save_file_set(file_set, &visitor)?;
        Ok(())
    }

    /// Saves the file set.
    fn save_file_set(&self, file_set: FileSet, project: &ProjectVisitor) -> Result<()> {
        let target = std::env::var("OUT_DIR")
            .ok()
            .map(PathBuf::from)
            .and_then(|path| path
                .ancestors()
                .skip(4)
                .next()
                .map(Path::to_path_buf))
            .unwrap_or(std::env::current_dir()?);
        let target_ligen_dir = target
            .join("ligen")
            .join(self.base_path());
        let project_dir = target_ligen_dir.join(&project.name().to_string());
        for (_path, file) in file_set.files {
            let file_path = project_dir.join(file.path);
            write_file(&file_path, &file.content)?;
        }
        Ok(())
    }
}

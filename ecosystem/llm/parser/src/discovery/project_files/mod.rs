use crate::prelude::*;
use crate::discovery::Files;
use std::path::PathBuf;

pub struct ProjectFiles {
    pub files: Files,
}

impl ProjectFiles {
    pub fn from_path(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        let files = walkdir::WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .map(|e| e.path().to_path_buf())
            .filter(|path| path.is_file())
            .filter(|path| !path.ancestors().any(|p| p.file_name().map(|f| f.to_string_lossy().eq(".git")).unwrap_or(false)))
            .collect::<Vec<PathBuf>>();
        let files = files.into();
        Ok(Self { files })
    }
}

impl std::fmt::Display for ProjectFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.files.fmt(f)
    }
}

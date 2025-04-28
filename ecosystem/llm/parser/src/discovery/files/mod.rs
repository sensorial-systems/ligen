use std::{ops::{Deref, DerefMut}, path::PathBuf};

#[derive(Debug, Default)]
pub struct Files(Vec<PathBuf>);

impl Files {
    pub fn new(files: Vec<PathBuf>) -> Self {
        Self(files)
    }
}

impl From<Vec<PathBuf>> for Files {
    fn from(files: Vec<PathBuf>) -> Self {
        Self(files)
    }
}

impl Deref for Files {
    type Target = Vec<PathBuf>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Files {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for Files {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.iter().map(|p| p.display().to_string()).collect::<Vec<_>>().join("\n"))
    }
}

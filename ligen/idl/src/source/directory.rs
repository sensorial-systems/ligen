use crate::prelude::*;

use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Directory {
    pub path: PathBuf,
    pub library_file: Option<PathBuf>
}

use std::path::PathBuf;

pub enum Source {
    Directory(PathBuf),
    Git(Git),
}

pub struct Git {
    pub remote: String,
    pub commit: String,
}
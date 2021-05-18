use std::path::PathBuf;

#[derive(Debug)]
/// Context Struct
pub struct Context {
    /// source_file field
    pub source_file: SourceFile,
}

#[derive(Debug)]
/// SourceFile Struct
pub struct SourceFile {
    /// is_real field
    pub is_real: bool,
    /// path field
    pub path: PathBuf,
}

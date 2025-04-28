use std::path::PathBuf;

use crate::{prelude::*, TypeDescriptor};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ProjectDiscovery {
    /// Name of the language.
    pub name: String,
    /// File extensions used by project documentation files.
    pub documentation_files_extensions: Vec<String>,
    /// File extensions used by project configuration files.
    // FIXME: Some files are extensionsless, like `Makefile`.
    pub project_files_extensions: Vec<String>,
    /// File extensions used by source files.
    pub source_files_extensions: Vec<String>,
    /// Example folders in the project.
    pub example_folders: Vec<PathBuf>,
}

impl TypeDescriptor for ProjectDiscovery {
    fn name() -> String {
        "ProjectDiscovery".to_string()
    }

    fn description() -> String {
        "General information about the project".to_string()
    }

    fn instruction() -> String {
        "You are an expert in project discovery. You will be given a list of file paths in the project for which you need to the programming language of the project and the file extensions used by project configuration files and source files".to_string()
    }

    fn input_description() -> String {
        "A list of file paths in the project".to_string()
    }
}
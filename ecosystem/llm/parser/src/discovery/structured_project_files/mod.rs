use crate::discovery::{Files, ProjectDiscovery, ProjectFiles};

#[derive(Debug)]
pub struct StructuredProjectFiles {
    pub documentation_files: Files,
    pub project_files: Files,
    pub source_files: Files
}

impl StructuredProjectFiles {
    pub fn new(project_files: &ProjectFiles, project_discovery: &ProjectDiscovery) -> Self {
        let documentation_files = Files::new(project_files.files.iter().filter(|p| project_discovery.documentation_files_extensions.contains(&p.extension().unwrap_or_default().to_string_lossy().to_string().replace(".", ""))).cloned().collect());
        let source_files = Files::new(project_files.files.iter().filter(|p| project_discovery.example_folders.iter().any(|f| !p.starts_with(f))).filter(|p| project_discovery.source_files_extensions.contains(&p.extension().unwrap_or_default().to_string_lossy().to_string().replace(".", ""))).cloned().collect());
        let project_files = Files::new(project_files.files.iter().filter(|p| project_discovery.project_files_extensions.contains(&p.extension().unwrap_or_default().to_string_lossy().to_string().replace(".", ""))).cloned().collect());
        Self { documentation_files, project_files, source_files }
    }

    pub fn documentation_files(&self) -> &Files {
        &self.documentation_files
    }

    pub fn project_files(&self) -> &Files {
        &self.project_files
    }

    pub fn source_files(&self) -> &Files {
        &self.source_files
    }
}

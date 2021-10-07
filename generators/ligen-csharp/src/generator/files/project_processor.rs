use super::*;

/// Project processor.
#[derive(Default, Clone, Copy, Debug)]
pub struct ProjectProcessor;

impl FileProcessorVisitor for ProjectProcessor {
    type Visitor = ProjectVisitor;

    fn process(&self, file_set: &mut FileSet, visitor: &Self::Visitor) {
        let file_name = format!("{}.csproj", PascalCase::from(visitor.current.name.clone()));
        let file = file_set.entry(&PathBuf::from(file_name));
        file.writeln(format!(include_str!("Template.csproj")));
    }

    fn post_process(&self, _file_set: &mut FileSet, _visitor: &Self::Visitor) {}
}

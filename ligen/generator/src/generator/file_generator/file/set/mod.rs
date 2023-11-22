//! File representation.

pub use super::*;

use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

/// Structure representing all the file set to be generated.
#[derive(Debug, Default)]
pub struct FileSet {
    pub(crate) files: HashMap<PathBuf, File>
}

impl FileSet {
    /// Creates a new FileSet.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a new file.
    pub fn insert(&mut self, file: File) {
        self.files.insert(file.path.clone(), file);
    }

    /// Returns an existing File assigned to an entry or creates a new one if it isn't present.
    pub fn entry(&mut self, path: impl AsRef<Path>) -> &mut File {
        self.files.entry(path.as_ref().to_path_buf()).or_insert(File::new(path))
    }
}

#[cfg(test)]
mod tests {
    use is_tree::IsTree;

    use crate::prelude::*;
    use crate::file_generator::{FileSection, SectionTemplate};

    use super::File;

    #[test]
    fn order() {
        let mut file = File::new("path");
        file.branch("b").write("B");
        file.branch("a").write("A");
        assert_eq!(file.to_string(), "BA");
    }

    #[test]
    fn section() {
        let mut section = FileSection::new("root");
        section.writeln("//! This is a Rust");
        section.writeln("//! documentation.");
        section.branch("sub1").write("//! This is a sub-section and ");
        section.branch("sub2").writeln("//! This is another sub-section.");
        section.branch("sub1").writeln("we can add to it later.");
        assert_eq!(section.to_string(), "//! This is a Rust\n//! documentation.\n//! This is a sub-section and we can add to it later.\n//! This is another sub-section.\n");
    }

    #[test]
    fn deep_section() {
        let mut section = FileSection::new("root");
        section.branch("attribute::begin").write("#[ligen(");
        section.branch("attribute::parameters").write("name = \"test\"");
        section.branch("attribute::parameters").write(", truth = true");
        section.branch("attribute::end").writeln(")]");
        assert_eq!(section.to_string(), "#[ligen(name = \"test\", truth = true)]\n");

        let mut section = FileSection::new("root");
        for name in ["attribute::begin", "attribute::parameters", "attribute::end"] {
            section.branch(name);
        }
        section.branch("attribute::begin").write("#[ligen(");
        section.branch("attribute::end").writeln(")]");
        section.branch("attribute::parameters").write("name = \"test\"");
        section.branch("attribute::parameters").write(", truth = true");
        assert_eq!(section.to_string(), "#[ligen(name = \"test\", truth = true)]\n");
    }

    #[test]
    fn template() -> Result<()> {
        let template = "[section(attribute::begin)][section(attribute::parameters)][section(attribute::end)]";
        let template = SectionTemplate::new("root", template);
        let mut section = FileSection::from_template(&template)?;
        section.branch("attribute::begin").write("#[ligen(");
        section.branch("attribute::end").writeln(")]");
        section.branch("attribute::parameters").write("name = \"test\"");
        section.branch("attribute::parameters").write(", truth = true");
        assert_eq!(section.to_string(), "#[ligen(name = \"test\", truth = true)]\n");
        Ok(())
    }

    #[test]
    fn template_with_content() -> Result<()> {
        let template = "before[section(begin)]content[section(end)]after";
        let template = SectionTemplate::new("root", template);
        let mut section = FileSection::from_template(&template)?;
        section.branch("begin").write("-begin-");
        section.branch("end").write("-end-");
        assert_eq!(section.to_string(), "before-begin-content-end-after");
        Ok(())
    }

    #[test]
    fn indexed_section() {
        let mut section = FileSection::new("root");
        section.indexed_write(0, "First");
        section.indexed_write(1, ", Third");
        section.indexed_write(1, ", Second");
        assert_eq!(section.to_string(), "First, Second, Third");
    }

    #[test]
    fn templated_sub_sections() -> Result<()> {
        let root = "[section(name)]\n[section(age)]\n";
        let name = "Name: [section(name)]";
        let age = "Age: [section(number)] years old";

        let mut root = SectionTemplate::new("root", root);
        root.add_branch(SectionTemplate::new("name", name));
        root.add_branch(SectionTemplate::new("age", age));

        let mut root = FileSection::from_template(&root)?;
        root.branch("name").write("John");
        root.branch("age").branch("number").write("42");
        assert_eq!(root.to_string(), "Name: John\nAge: 42 years old\n");
        Ok(())
    }

    #[test]
    fn indentation() {
        let mut section = FileSection::new("root");
        section.writeln("fn main() {");
        let body = section.indented_branch("body");
        body.writeln("if true {");
        let condition_body = body.indented_branch("condition");
        condition_body.writeln("println!(\"Hello, world!\");");
        body.writeln("}");
        section.writeln("}");
        assert_eq!(section.to_string(), "fn main() {\n    if true {\n        println!(\"Hello, world!\");\n    }\n}\n");
    }
}

//! File representation.

use crate::prelude::*;
use ligen_utils::fs::write_file;
use std::path::{Path, PathBuf};
use std::collections::{HashMap, BTreeMap};

/// Structure representing a file path and its content.
#[derive(Debug, Clone, PartialEq, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct File {
    /// File path.
    pub path: PathBuf,
    /// File Section.
    #[shrinkwrap(main_field)]
    pub section: FileSection
}

impl File {
    /// Creates a new file with the specified path and content.
    pub fn new(path: impl AsRef<std::path::Path>) -> Self {
        let path = path.as_ref().to_path_buf();
        let section = Default::default();
        Self { path, section }
    }

    /// Saves the file.
    pub fn save(&self) -> Result<()> {
        write_file(&self.path, self.content())
    }
}


#[derive(Default, Debug, Clone, PartialEq)]
pub struct FileSection {
    /// File section content.
    content: Vec<String>,
    sections: BTreeMap<String, FileSection>,
    order: Vec<String>
}

impl FileSection {
    /// Creates a new FileSection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new FileSection from a template.
    pub fn from_template(template: impl AsRef<str>) -> Result<Self> {
        let mut section = Self::new();
        let template = template.as_ref();
        const SECTION_START: &str = "[section(";
        const SECTION_END: &str = ")]";
        for (index, _) in template.match_indices(SECTION_START) {
            let index = index + SECTION_START.len();
            let index_end = template[index..]
                .find(SECTION_END)
                .ok_or_else(|| Error::Message("Failed to parse template: missing section end.".to_string()))? + index;
            let section_name = &template[index..index_end];
            section.section(section_name);
        }
        Ok(section)
    }

    /// Gets content.
    pub fn content(&self) -> String {
        let mut content = self.content.join("");
        for section in &self.order {
            if let Some(section) = self.sections.get(section) {
                content.push_str(&section.content());
            }
        }
        content
    }
    
    /// Gets or creates a new section with the specified name.
    pub fn section(&mut self, name: impl AsRef<str>) -> &mut FileSection {
        self
            .sections
            .entry(name.as_ref().to_string())
            .or_insert_with(|| {
                self.order.push(name.as_ref().to_string());
                Default::default()
            })
    }

    /// Writes the content to the file section at the specified index.
    pub fn indexed_write<S: AsRef<str>>(&mut self, index: usize, content: S) {
        self.content.insert(index, content.as_ref().to_string())
    }

    /// Writes the content to the file section at the specified index and adds a new line.
    pub fn indexed_writeln<S: AsRef<str>>(&mut self, index: usize, content: S) {
        let mut string = content.as_ref().to_string();
        string.push('\n');
        self.indexed_write(index, string);
    }

    /// Writes the content to the file buffer.
    pub fn write<S: AsRef<str>>(&mut self, content: S) {
        self.content.push(content.as_ref().to_string());
    }

    /// Writes the content to the file buffer and adds a new line.
    pub fn writeln<S: AsRef<str>>(&mut self, content: S) {
        let mut string = content.as_ref().to_string();
        string.push('\n');
        self.content.push(string);
    }    
}

/// Structure representing all the file set to be generated.
#[derive(Debug, Default, Clone)]
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
    use crate::prelude::*;
    use crate::file_generator::FileSection;

    use super::File;

    #[test]
    fn order() {
        let mut file = File::new("path");
        file.section("b").write("B");
        file.section("a").write("A");
        assert_eq!(file.content(), "BA");
    }

    #[test]
    fn section() {
        let mut section = FileSection::new();
        section.writeln("//! This is a Rust");
        section.writeln("//! documentation.");
        section.section("sub1").write("//! This is a sub-section and ");
        section.section("sub2").writeln("//! This is another sub-section.");
        section.section("sub1").writeln("we can add to it later.");
        assert_eq!(section.content(), "//! This is a Rust\n//! documentation.\n//! This is a sub-section and we can add to it later.\n//! This is another sub-section.\n");
    }

    #[test]
    fn deep_section() {
        let mut section = FileSection::new();
        section.section("attribute::begin").write("#[ligen(");
        section.section("attribute::parameters").write("name = \"test\"");
        section.section("attribute::parameters").write(", truth = true");
        section.section("attribute::end").writeln(")]");
        assert_eq!(section.content(), "#[ligen(name = \"test\", truth = true)]\n");

        let mut section = FileSection::new();
        for name in ["attribute::begin", "attribute::parameters", "attribute::end"] {
            section.section(name);
        }
        section.section("attribute::begin").write("#[ligen(");
        section.section("attribute::end").writeln(")]");
        section.section("attribute::parameters").write("name = \"test\"");
        section.section("attribute::parameters").write(", truth = true");
        assert_eq!(section.content(), "#[ligen(name = \"test\", truth = true)]\n");
    }

    #[test]
    fn template() -> Result<()> {
        let template = "[section(attribute::begin)][section(attribute::parameters)][section(attribute::end)]";
        let mut section = FileSection::from_template(template)?;
        assert_eq!(section.order, vec!["attribute::begin", "attribute::parameters", "attribute::end"]);
        section.section("attribute::begin").write("#[ligen(");
        section.section("attribute::end").writeln(")]");
        section.section("attribute::parameters").write("name = \"test\"");
        section.section("attribute::parameters").write(", truth = true");
        assert_eq!(section.content(), "#[ligen(name = \"test\", truth = true)]\n");
        Ok(())
    }

    // #[test]
    // fn template_with_content() -> Result<()> {
    //     let template = "before[section(begin)]content[section(end)]after";
    //     let mut section = FileSection::from_template(template)?;
    //     assert_eq!(section.order, vec!["begin", "end"]);
    //     section.section("begin").write("-begin-");
    //     section.section("end").write("-end-");
    //     assert_eq!(section.content(), "before-begin-content-end-after");
    //     Ok(())
    // }

    #[test]
    fn indexed_section() {
        let mut section = FileSection::new();
        section.indexed_write(0, "First");
        section.indexed_write(1, ", Third");
        section.indexed_write(1, ", Second");
        assert_eq!(section.content(), "First, Second, Third");
    }
}

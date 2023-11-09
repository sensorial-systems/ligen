//! File representation.

use crate::prelude::*;
use ligen_utils::fs::write_file;
use std::fmt::{Display, Debug};
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

/// Structure representing a file path and its content.
#[derive(Debug, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct File {
    /// File path.
    pub path: PathBuf,
    /// File Section.
    #[shrinkwrap(main_field)]
    pub section: FileSection
}

impl File {
    /// Creates a new file from a template.
    pub fn from_template(path: impl AsRef<Path>, template: impl AsRef<str>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let section = FileSection::from_template("root", template)?;
        Ok(Self { path, section })
    }

    /// Creates a new file with the specified path and content.
    pub fn new(path: impl AsRef<std::path::Path>) -> Self {
        let path = path.as_ref().to_path_buf();
        let section = FileSection::new("root");
        Self { path, section }
    }

    /// Saves the file.
    pub fn save(&self) -> Result<()> {
        write_file(&self.path, self.to_string())
    }
}

pub trait FileContent: Display + Debug {
    fn as_string(&self) -> Option<&String> {
        None
    }
    fn as_string_mut(&mut self) -> Option<&mut String> {
        None
    }
    fn as_section(&self) -> Option<&FileSection> {
        None
    }
    fn as_section_mut(&mut self) -> Option<&mut FileSection> {
        None
    }
}
impl FileContent for String {
    fn as_string(&self) -> Option<&String> {
        Some(self)
    }
    fn as_string_mut(&mut self) -> Option<&mut String> {
        Some(self)
    }
}
impl FileContent for FileSection {
    fn as_section(&self) -> Option<&FileSection> {
        Some(self)
    }
    fn as_section_mut(&mut self) -> Option<&mut FileSection> {
        Some(self)
    }
}

impl Display for FileSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for file_content in &self.content {
            write!(f, "{}", file_content)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct FileSection {
    /// File section name.
    name: String,
    /// File section content.
    content: Vec<Box<dyn FileContent>>
}

impl FileSection {
    /// Creates a new FileSection.
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        let content = Default::default();
        Self { name, content }
    }

    /// Creates a new FileSection from a template.
    pub fn from_template(name: impl Into<String>, template: impl AsRef<str>) -> Result<Self> {
        let mut section = Self::new(name);
        let template = template.as_ref();
        let sections = Self::get_sections_ranges(template)?;
        section.write_from_template(template, sections);

        Ok(section)
    }

    /// Gets the section name.
    pub fn find_section(&mut self, name: impl AsRef<str>) -> Option<(usize, &mut FileSection)> {
        let name = name.as_ref();
        self.content
            .iter_mut()
            .enumerate()
            .find_map(|(index, content)| {
                content
                    .as_section_mut()
                    .and_then(|section|
                        if section.name == name {
                            Some((index, section))
                        } else {
                            None
                        }
                    )
            })
    }

    /// Set section.
    pub fn set_section(&mut self, section: FileSection) {
        if let Some((_, old_section)) = self.find_section(&section.name) {
            *old_section = section;
        } else {
            self.content.push(Box::new(section));
        }
    }

    /// Gets or creates a new section with the specified name.
    pub fn section(&mut self, name: impl AsRef<str>) -> &mut FileSection {
        let (index, exists) = self
            .find_section(name.as_ref())
            .map(|(index, _)| (index, true))
            .unwrap_or((0, false));
        if exists {
            self
                .content
                .get_mut(index)
                .unwrap()
                .as_section_mut()
                .unwrap()
        } else {
            let section = FileSection::new(name.as_ref());
            self.set_section(section);
            self.content.last_mut().unwrap().as_section_mut().unwrap()
        }
    }

    /// Writes the content to the file section at the specified index.
    pub fn indexed_write<S: Into<String>>(&mut self, index: usize, content: S) {
        self.content.insert(index, Box::new(content.into()))
    }

    /// Writes the content to the file section at the specified index and adds a new line.
    pub fn indexed_writeln<S: Into<String>>(&mut self, index: usize, content: S) {
        let mut string = content.into();
        string.push('\n');
        self.indexed_write(index, string);
    }

    /// Writes the content to the file buffer.
    pub fn write<S: Into<String>>(&mut self, content: S) {
        self.content.push(Box::new(content.into()));
    }

    /// Writes the content to the file buffer and adds a new line.
    pub fn writeln<S: Into<String>>(&mut self, content: S) {
        let mut string = content.into();
        string.push('\n');
        self.content.push(Box::new(string));
    }    
}

impl FileSection {
    /// Section start.
    const SECTION_START: &'static str = "[section(";
    /// Section end.
    const SECTION_END: &'static str = ")]";

    /// Gets the sections ranges.
    fn get_sections_ranges(template: impl AsRef<str>) -> Result<Vec<Range<usize>>> {
        let template = template.as_ref();
        let mut sections = Vec::new();
        for (index, _) in template.match_indices(Self::SECTION_START) {
            let index_start = index;
            let index_end = template[index_start..]
                .find(Self::SECTION_END)
                .ok_or_else(|| Error::Message("Failed to parse template: missing section end.".to_string()))?;
            let index_end = index_end + index_start + Self::SECTION_END.len();
            sections.push(index_start..index_end);
        }
        Ok(sections)        
    }

    /// Registers the sections and writes the text in-between them. See the `template_with_content` test.
    fn write_from_template(&mut self, template: impl AsRef<str>, sections: impl IntoIterator<Item = Range<usize>>) {
        let template = template.as_ref();
        let mut start = 0;
        for section in sections {
            let before = &template[start..section.start];
            if !before.is_empty() {
                self.write(before);
            }
            start = section.end;
            let section = &template[(section.start + Self::SECTION_START.len())..(section.end - Self::SECTION_END.len())];
            self.section(section);
        }
        let after = &template[start..];
        if !after.is_empty() {
            self.write(after);
        }
    }
}

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
    use crate::prelude::*;
    use crate::file_generator::FileSection;

    use super::File;

    #[test]
    fn order() {
        let mut file = File::new("path");
        file.section("b").write("B");
        file.section("a").write("A");
        assert_eq!(file.to_string(), "BA");
    }

    #[test]
    fn section() {
        let mut section = FileSection::new("root");
        section.writeln("//! This is a Rust");
        section.writeln("//! documentation.");
        section.section("sub1").write("//! This is a sub-section and ");
        section.section("sub2").writeln("//! This is another sub-section.");
        section.section("sub1").writeln("we can add to it later.");
        assert_eq!(section.to_string(), "//! This is a Rust\n//! documentation.\n//! This is a sub-section and we can add to it later.\n//! This is another sub-section.\n");
    }

    #[test]
    fn deep_section() {
        let mut section = FileSection::new("root");
        section.section("attribute::begin").write("#[ligen(");
        section.section("attribute::parameters").write("name = \"test\"");
        section.section("attribute::parameters").write(", truth = true");
        section.section("attribute::end").writeln(")]");
        assert_eq!(section.to_string(), "#[ligen(name = \"test\", truth = true)]\n");

        let mut section = FileSection::new("root");
        for name in ["attribute::begin", "attribute::parameters", "attribute::end"] {
            section.section(name);
        }
        section.section("attribute::begin").write("#[ligen(");
        section.section("attribute::end").writeln(")]");
        section.section("attribute::parameters").write("name = \"test\"");
        section.section("attribute::parameters").write(", truth = true");
        assert_eq!(section.to_string(), "#[ligen(name = \"test\", truth = true)]\n");
    }

    #[test]
    fn template() -> Result<()> {
        let template = "[section(attribute::begin)][section(attribute::parameters)][section(attribute::end)]";
        let mut section = FileSection::from_template("root", template)?;
        section.section("attribute::begin").write("#[ligen(");
        section.section("attribute::end").writeln(")]");
        section.section("attribute::parameters").write("name = \"test\"");
        section.section("attribute::parameters").write(", truth = true");
        assert_eq!(section.to_string(), "#[ligen(name = \"test\", truth = true)]\n");
        Ok(())
    }

    #[test]
    fn template_with_content() -> Result<()> {
        let template = "before[section(begin)]content[section(end)]after";
        let mut section = FileSection::from_template("root", template)?;
        section.section("begin").write("-begin-");
        section.section("end").write("-end-");
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

    struct SectionTemplate {
        name: String,
        template: String,
        children: Vec<Self>
    }

    impl SectionTemplate {
        pub fn new(name: impl Into<String>, template: impl Into<String>) -> Self {
            let name = name.into();
            let template = template.into();
            let children = Default::default();
            Self { name, template, children }
        }

        pub fn set_child(&mut self, template: impl Into<Self>) {
            let template = template.into();
            self.children.push(template);
        }
    }

    #[test]
    fn templated_sub_sections() -> Result<()> {
        let root = "[section(name)]\n[section(age)]\n";
        let name = "Name: [section(name)]";
        let age = "Age: [section(number)] years old";

        let mut template = SectionTemplate::new("root", root);
        template.set_child(SectionTemplate::new("name", name));
        template.set_child(SectionTemplate::new("age", age));

        let name = FileSection::from_template("name", name)?;
        let age = FileSection::from_template("age", age)?;
        let mut root = FileSection::from_template("root", root)?;
        root.set_section(name);
        root.set_section(age);
        root.section("name").write("John");
        root.section("age").section("number").write("42");
        for (index, content) in root.content.iter().enumerate() {
            println!("{} - {:?}", index, content)
        }
        assert_eq!(root.to_string(), "Name: John\nAge: 42 years old\n");
        Ok(())
    }
}

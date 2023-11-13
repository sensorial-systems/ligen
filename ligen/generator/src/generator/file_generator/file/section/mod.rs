use crate::prelude::*;

pub mod content;
pub mod template;

pub use content::*;
use ligen_utils::tree::IsTree;
pub use template::*;

use std::ops::Range;

#[derive(Debug)]
pub struct FileSection {
    /// File section name.
    pub name: String,
    /// File section content.
    pub content: Vec<Box<dyn FileSectionContent>>
}

impl FileSection {
    /// Creates a new FileSection.
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        let content = Default::default();
        Self { name, content }
    }

    /// Creates a new FileSection from a template.
    pub fn from_template(template: &SectionTemplate) -> Result<Self> {
        let mut section = Self::new(&template.name);
        let sections = Self::get_sections_ranges(template)?;
        section.write_from_template(template, sections)?;

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
    fn get_sections_ranges(template: &SectionTemplate) -> Result<Vec<Range<usize>>> {
        let template = template.content.as_str();
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
    fn write_from_template(&mut self, template: &SectionTemplate, sections: impl IntoIterator<Item = Range<usize>>) -> Result<()> {
        let mut start = 0;
        for section in sections {
            let before = &template.content[start..section.start];
            if !before.is_empty() {
                self.write(before);
            }
            start = section.end;
            let section = &template.content[(section.start + Self::SECTION_START.len())..(section.end - Self::SECTION_END.len())];
            let section = if let Some(template) = template.get(section) {
                FileSection::from_template(template)?
            } else {
                FileSection::new(section)
            };
            self.set_section(section);
        }
        let after = &template.content[start..];
        if !after.is_empty() {
            self.write(after);
        }
        Ok(())
    }
}

impl std::fmt::Display for FileSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for file_content in &self.content {
            write!(f, "{}", file_content)?;
        }
        Ok(())
    }
}

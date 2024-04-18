use crate::file_generator::file::section::FileSection;
use std::fmt::{Debug, Display};

pub trait FileSectionContent: Display + Debug {
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

impl FileSectionContent for String {
    fn as_string(&self) -> Option<&String> {
        Some(self)
    }
    fn as_string_mut(&mut self) -> Option<&mut String> {
        Some(self)
    }
}

impl FileSectionContent for FileSection {
    fn as_section(&self) -> Option<&FileSection> {
        Some(self)
    }
    fn as_section_mut(&mut self) -> Option<&mut FileSection> {
        Some(self)
    }
}

impl<'a> TryFrom<&'a Box<dyn FileSectionContent>> for &'a FileSection {
    type Error = ();

    fn try_from(value: &'a Box<dyn FileSectionContent>) -> std::result::Result<Self, Self::Error> {
        value
            .as_section()
            .ok_or(())
    }
}

impl<'a> TryFrom<&'a mut Box<dyn FileSectionContent>> for &'a mut FileSection {
    type Error = ();

    fn try_from(value: &'a mut Box<dyn FileSectionContent>) -> std::result::Result<Self, Self::Error> {
        value
            .as_section_mut()
            .ok_or(())
    }
}
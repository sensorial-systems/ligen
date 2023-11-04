pub use crate::prelude::*;

use std::path::PathBuf;
use ligen_ir::{Identifier, Path};

pub trait StringEditable {
    fn as_string(&self) -> String;
    fn update(&mut self, string: impl AsRef<str>) -> bool;
}

impl StringEditable for PathBuf {
    fn as_string(&self) -> String {
        self.display().to_string()
    }

    fn update(&mut self, string: impl AsRef<str>) -> bool {
        if let Ok(path_buf) = PathBuf::try_from(string.as_ref()) {
            *self = path_buf;
            true
        } else {
            false
        }
    }
}

impl StringEditable for Path {
    fn as_string(&self) -> String {
        self.to_string_with_separator("::")
    }

    fn update(&mut self, string: impl AsRef<str>) -> bool {
        *self = Path::from_string_with_separator(string.as_ref(), "::");
        true
    }
}

impl StringEditable for Identifier {
    fn as_string(&self) -> String {
        self.to_string()
    }

    fn update(&mut self, string: impl AsRef<str>) -> bool {
        self.name = string.as_ref().into();
        true
    }
}
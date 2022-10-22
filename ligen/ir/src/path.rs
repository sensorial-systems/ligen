use crate::Identifier;
use crate::prelude::*;
use std::path::PathBuf;

/// A fully qualified path.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Path {
    /// The path segments.
    pub segments: Vec<Identifier>
}

impl Path {
    /// Converts to string with specified separator.
    pub fn to_string(&self, separator: &str) -> String {
        self
            .segments
            .clone()
            .into_iter()
            .map(|identifier| identifier.name)
            .collect::<Vec<_>>()
            .join(&separator)
    }

    /// Get the last segment of the path.
    pub fn last(&self) -> Identifier {
        self.segments.last().unwrap().clone()
    }

    /// Join the current path with another path.
    pub fn join<T: Into<Path>>(self, another: T) -> Self {
        let mut this = self;
        this.segments.append(&mut another.into().segments);
        this
    }

    /// Returns the Path without the first segment.
    pub fn without_first(mut self) -> Self {
        self.pop_front();
        self
    }

    /// Returns the Path without the last segment.
    pub fn without_last(mut self) -> Self {
        self.pop_back();
        self
    }

    /// Removes the first element and returns it, or None if the Path is empty.
    pub fn pop_front(&mut self) -> Option<Identifier> {
        if self.segments.len() > 0 {
            Some(self.segments.remove(0))
        } else {
            None
        }
    }

    /// Removes the last element and returns it, or None if the Path is empty.
    pub fn pop_back(&mut self) -> Option<Identifier> {
        self.segments.pop()
    }
}

impl<I: Into<Identifier>> From<Vec<I>> for Path {
    fn from(from: Vec<I>) -> Self {
        let segments = from.into_iter().map(|x| x.into()).collect();
        Self { segments }
    }
}

impl From<&str> for Path {
    fn from(string: &str) -> Path {
        let segments = string
            .split("::")
            .into_iter()
            .map(|segment| Identifier::new(segment))
            .collect();
        Self { segments }
    }
}

impl From<String> for Path {
    fn from(string: String) -> Path {
        string.as_str().into()
    }
}

impl From<PathBuf> for Path {
    fn from(path: PathBuf) -> Self {
        let segments = path
            .iter()
            .filter_map(|segment| segment.to_str())
            .map(|segment| segment.into())
            .collect();
        Self { segments }
    }
}

impl From<Identifier> for Path {
    fn from(identifier: Identifier) -> Self {
        let segments = vec![identifier];
        Self { segments }
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let segments: Vec<_> = self.segments.iter().map(|identifier| identifier.to_string()).collect();
        f.write_str(&segments.join("::"))
    }
}
pub mod path_segment;

use crate::Identifier;
use crate::Type;
use crate::prelude::*;
use std::path::PathBuf;

use is_tree::IntoIterTypeMut;
use is_tree::TypeIteratorMut;
pub use path_segment::*;

#[cfg(any(test, feature = "mocks"))]
pub mod mock;

/// A fully qualified path.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Path {
    /// The path segments.
    pub segments: Vec<PathSegment>
}

impl Path {
    /// Create a new `Path`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Is empty.
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    /// Get `Path` from a `string` with a specified `separator`.
    pub fn from_string_with_separator(string: &str, separator: impl AsRef<str>) -> Self {
        let separator = separator.as_ref();
        let segments = string
            .split(separator)
            .map(PathSegment::from)
            .collect();
        Self { segments }

    }

    /// Convert the `Path` to a string with a specified separator.
    pub fn to_string_with_separator(&self, separator: impl AsRef<str>) -> String {
        let separator = separator.as_ref();
        let segments: Vec<_> = self.segments.iter().map(|identifier| identifier.to_string()).collect();
        segments.join(separator)
    }

    /// Get the first segment's reference.
    pub fn first(&self) -> &PathSegment {
        self.segments.first().unwrap()
    }

    /// Get the first segment's mutable reference.
    pub fn first_mut(&mut self) -> &mut PathSegment {
        self.segments.first_mut().unwrap()
    }

    /// Get the last segment's reference.
    pub fn last(&self) -> &PathSegment {
        self.segments.last().unwrap()
    }

    /// Get the last segment's mutable reference.
    pub fn last_mut(&mut self) -> &mut PathSegment {
        self.segments.last_mut().unwrap()
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
    pub fn pop_front(&mut self) -> Option<PathSegment> {
        if !self.segments.is_empty() {
            Some(self.segments.remove(0))
        } else {
            None
        }
    }

    /// Removes the last element and returns it, or None if the Path is empty.
    pub fn pop_back(&mut self) -> Option<PathSegment> {
        self.segments.pop()
    }

    /// Pushes the given segment onto the end of the Path.
    pub fn push_front(&mut self, segment: impl Into<PathSegment>) {
        self.segments.insert(0, segment.into());
    }

    /// Pushes the given segment onto the end of the Path.
    pub fn push_back(&mut self, segment: impl Into<PathSegment>) {
        self.segments.push(segment.into());
    }
}

impl<I: Into<Identifier>> From<Vec<I>> for Path {
    fn from(from: Vec<I>) -> Self {
        let segments = from
            .into_iter()
            .map(|x| x.into().into())
            .collect();
        Self { segments }
    }
}

impl From<PathSegment> for Path {
    fn from(segment: PathSegment) -> Self {
        Self { segments: vec![segment] }
    }
}

impl From<&[&str]> for Path {
    fn from(from: &[&str]) -> Self {
        let segments = from
            .iter()
            .map(|x| (*x).into())
            .collect();
        Self { segments }
    }

}

impl From<&str> for Path {
    fn from(string: &str) -> Path {
        Self::from_string_with_separator(string, "::")
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

impl From<Path> for PathBuf {
    fn from(path: Path) -> Self {
        let mut path_buf = PathBuf::new();
        for segment in path.segments {
            path_buf = path_buf.join(segment.identifier.name);
        }
        path_buf
    }
}

impl From<Identifier> for Path {
    fn from(identifier: Identifier) -> Self {
        let segments = vec![identifier.into()];
        Self { segments }
    }
}

impl<'a> From<is_tree::Path<'a, Identifier>> for Path {
    fn from(path: is_tree::Path<'a, Identifier>) -> Self {
        let segments = path
            .segments
            .iter()
            .map(|segment| segment.clone().into())
            .collect();
        Self { segments }
    }

}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.to_string_with_separator("::").as_str())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn path_from_string() {
        let path: Path = "std::convert::TryFrom".into();
        let segments: Vec<_> = vec!["std", "convert", "TryFrom"]
            .into_iter()
            .map(PathSegment::from)
            .collect();
        assert_eq!(path.segments, segments);
    }
}

impl IntoIterTypeMut<Type> for Path {
    fn into_type_iterator<'a>(&'a mut self) -> TypeIteratorMut<'a, Type> {
        self
            .segments
            .iter_mut()
            .flat_map(|segment| segment.into_type_iterator())
            .collect::<Vec<_>>()
            .into()
    }
}
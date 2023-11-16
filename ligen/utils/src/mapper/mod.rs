use std::hash::Hash;
use bimap::BiMap;

pub struct LanguageMap<T>
where T: Eq + Hash
{
    left: String,
    right: String,
    map: BiMap<T, T>
}

impl<T> LanguageMap<T>
where T: Eq + Hash
{
    pub fn new(left: impl Into<String>, right: impl Into<String>) -> Self {
        let left = left.into();
        let right = right.into();
        let map = Default::default();
        Self { left, right, map }
    }

    pub fn insert(&mut self, left: impl Into<T>, right: impl Into<T>) {
        self.map.insert(left.into(), right.into());
    }

    pub fn get(&self, language: impl PartialEq<String>, value: &T) -> Option<&T> {
        if language == self.left {
            self.map.get_by_left(value)
        } else if language == self.right {
            self.map.get_by_right(value)
        } else {
            None
        }
    }
}

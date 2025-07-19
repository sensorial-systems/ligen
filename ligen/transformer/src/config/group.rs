use std::collections::HashMap;

use ligen_idl::{Literal, Path};

use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Value {
    Literal(Literal),
    Group(Group)
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Group {
    #[serde(flatten)]
    map: HashMap<String, Value>
}

impl Group {
    pub fn iter(&self) -> impl Iterator<Item = (Path, Literal)> {
        self.map
            .clone()
            .into_iter()
            .flat_map(|(key, value)| {
                match value {
                    Value::Literal(literal) => {
                        vec![(Path::from(key), literal)]
                    },
                    Value::Group(group) => {
                        group
                            .iter()
                            .map(|(path, literal)| {
                                let mut path = path.clone();
                                path.push_front(key.clone());
                                (path, literal.clone())
                            })
                            .collect()
                    }
                }
            })
    }
}

impl Group {
    /// Sets the value at the given path.
    pub fn set<P: Into<Path>, L: Into<Literal>>(&mut self, path: P, value: L) {
        let mut path = path.into();
        if let Some(word) = path.pop_front() {
            if path.is_empty() {
                self.map.insert(word.identifier.name, Value::Literal(value.into()));
            } else {
                let group = self.map
                    .entry(word.identifier.name)
                    .or_insert_with(|| Value::Group(Group::default()));
                if let Value::Group(group) = group {
                    group.set(path, value);
                }
            }
        }
    }

    /// Gets the value at the given path.
    pub fn get<P: Into<Path>>(&self, path: P) -> Option<&Literal> {
        let mut path = path.into();
        if let Some(word) = path.pop_front() {
            match self
                .map
                .get(&word.identifier.name) {
                Some(Value::Literal(literal)) => {
                    if path.is_empty() {
                        Some(literal)
                    } else {
                        None
                    }
                },
                Some(Value::Group(group)) => group.get(path),
                None => None
            }
        } else {
            None
        }
    }
}

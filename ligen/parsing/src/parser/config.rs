
use crate::prelude::*;

use std::collections::HashMap;

use ligen_ir::{Literal, Path};

#[derive(Default, Serialize, Deserialize)]
pub struct ParserConfig {
    #[serde(flatten)]
    map: Group
}

impl ParserConfig {
    /// Whether to parse all symbols or only the ones that are explicitly marked as such.
    pub fn only_parse_symbols(&self) -> bool {
        self.get("ligen::only-parse-symbols")
            .and_then(|literal| literal.as_boolean())
            .cloned()
            .unwrap_or(false)
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
enum Value {
    Literal(Literal),
    Group(Group)
}

#[derive(Default, Serialize, Deserialize, Clone)]
struct Group {
    #[serde(flatten)]
    map: HashMap<String, Value>
}

impl Group {
    /// Sets the value at the given path.
    fn set<P: Into<Path>, L: Into<Literal>>(&mut self, path: P, value: L) {
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
    fn get<P: Into<Path>>(&self, path: P) -> Option<&Literal> {
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

impl TryFrom<&str> for ParserConfig {
    type Error = toml::de::Error;
    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        toml::from_str(value)
    }
}

impl ParserConfig {
    /// Gets the value at the given path.
    pub fn get<P: Into<Path>>(&self, path: P) -> Option<&Literal> {
        self.map.get(path)
    }

    /// Sets the value at the given path.
    pub fn set<P: Into<Path>, L: Into<Literal>>(&mut self, path: P, value: L) {
        self.map.set(path, value);
    }
}

#[cfg(test)]
mod tests {
    use super::ParserConfig;

    fn config() -> ParserConfig {
        let mut config = ParserConfig::try_from(r#"
            [ligen]
            parse-all = false"#
        ).unwrap();
        config.set("ligen::default-name", "library");
        config
    }

    #[test]
    fn parser_config() {
        let config = config();
        assert_eq!(config.get("ligen::parse_all"), None);
        assert_eq!(config.get("ligen::parse-all"), Some(&false.into()));
        assert_eq!(config.get("ligen::default-name"), Some(&"library".into()));
    }
}
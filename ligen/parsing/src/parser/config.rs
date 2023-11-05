
use crate::prelude::*;

use std::collections::HashMap;

use ligen_ir::{Literal, Path};

#[derive(Default, Serialize, Deserialize)]
pub struct ParserConfig {
    #[serde(flatten)]
    map: Group
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum Value {
    Literal(Literal),
    Group(Group)
}

#[derive(Default, Serialize, Deserialize)]
struct Group {
    #[serde(flatten)]
    map: HashMap<String, Value>
}

impl Group {
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
    pub fn get<P: Into<Path>>(&self, path: P) -> Option<&Literal> {
        self.map.get(path)
    }
}

#[cfg(test)]
mod tests {
    use super::ParserConfig;

    fn config() -> ParserConfig {
        ParserConfig::try_from(r#"
            [ligen]
            parse-all = false
            default-name = "library""#
        ).unwrap()
    }

    #[test]
    fn parser_config() {
        let config = config();
        assert_eq!(config.get("ligen::parse_all"), None);
        assert_eq!(config.get("ligen::parse-all"), Some(&false.into()));
        assert_eq!(config.get("ligen::default-name"), Some(&"library".into()));
    }
}
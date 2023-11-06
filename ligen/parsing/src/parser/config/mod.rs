mod group;
pub use group::*;

use crate::prelude::*;

use ligen_ir::{Literal, Path};

#[derive(Serialize, Deserialize, Clone)]
pub struct ParserConfig {
    #[serde(flatten)]
    map: Group
}

impl Default for ParserConfig {
    fn default() -> Self {
        let map = Default::default();
        let mut config = Self { map };
        config.set_only_parse_symbols(false);
        config
    }
}

impl ParserConfig {
    pub fn iter(&self) -> impl Iterator<Item = (Path, Literal)> {
        self.map.iter()
    }
}

pub trait ParserConfigGet {
    /// Gets the value at the given path.
    fn get<P: Into<Path>>(&self, path: P) -> Option<&Literal>;

    /// Whether to parse all symbols or only the ones that are explicitly marked as such.
    fn get_only_parse_symbols(&self) -> bool {
        self.get("ligen::only-parse-symbols")
            .and_then(|literal| literal.as_boolean())
            .cloned()
            .unwrap_or(false)
    }    
}

pub trait ParserConfigSet {    
    /// Sets the value at the given path.
    fn set<P: Into<Path>, L: Into<Literal>>(&mut self, path: P, value: L);
    
    /// Sets whether to parse all symbols or only the ones that are explicitly marked as such.
    fn set_only_parse_symbols(&mut self, value: bool) {
        self.set("ligen::only-parse-symbols", value);
    }
}

impl ParserConfig {
    /// Creates a new parser config.
    pub fn new() -> Self {
        Default::default()
    }
}

impl ParserConfigGet for ParserConfig {
    /// Gets the value at the given path.
    fn get<P: Into<Path>>(&self, path: P) -> Option<&Literal> {
        self.map.get(path)
    }
}

impl ParserConfigGet for &ParserConfig {
    /// Gets the value at the given path.
    fn get<P: Into<Path>>(&self, path: P) -> Option<&Literal> {
        self.map.get(path)
    }
}

impl ParserConfigGet for &mut ParserConfig {
    /// Gets the value at the given path.
    fn get<P: Into<Path>>(&self, path: P) -> Option<&Literal> {
        self.map.get(path)
    }
}

impl ParserConfigSet for ParserConfig {
    /// Sets the value at the given path.
    fn set<P: Into<Path>, L: Into<Literal>>(&mut self, path: P, value: L) {
        self.map.set(path, value);
    }
}

impl ParserConfigSet for &mut ParserConfig {
    /// Sets the value at the given path.
    fn set<P: Into<Path>, L: Into<Literal>>(&mut self, path: P, value: L) {
        self.map.set(path, value);
    }
}

impl TryFrom<&str> for ParserConfig {
    type Error = toml::de::Error;
    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        toml::from_str(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

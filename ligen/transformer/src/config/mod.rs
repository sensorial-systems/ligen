mod group;
pub use group::*;

use crate::prelude::*;

use ligen_idl::{Literal, Path};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(flatten)]
    map: Group
}

impl Default for Config {
    fn default() -> Self {
        let map = Default::default();
        let mut config = Self { map };
        config.set_only_parse_symbols(false);
        config
    }
}

impl Config {
    pub fn iter(&self) -> impl Iterator<Item = (Path, Literal)> {
        self.map.iter()
    }

    /// Sets whether to parse all symbols or only the ones that are explicitly marked as such.
    pub fn set_only_parse_symbols(&mut self, value: bool) {
        self.set("ligen::only-parse-symbols", value);
    }    

    /// Whether to parse all symbols or only the ones that are explicitly marked as such.
    pub fn get_only_parse_symbols(&self) -> bool {
        self.get("ligen::only-parse-symbols")
            .and_then(|literal| literal.as_boolean())
            .cloned()
            .unwrap_or(false)
    }
}

pub trait ConfigGet {
    /// Gets the value at the given path.
    fn get<P: Into<Path>>(&self, path: P) -> Option<&Literal>;
}

pub trait ConfigSet {    
    /// Sets the value at the given path.
    fn set<P: Into<Path>, L: Into<Literal>>(&mut self, path: P, value: L);    
}

impl Config {
    /// Creates a new parser config.
    pub fn new() -> Self {
        Default::default()
    }
}

impl ConfigGet for Config {
    /// Gets the value at the given path.
    fn get<P: Into<Path>>(&self, path: P) -> Option<&Literal> {
        self.map.get(path)
    }
}

impl ConfigGet for &Config {
    /// Gets the value at the given path.
    fn get<P: Into<Path>>(&self, path: P) -> Option<&Literal> {
        self.map.get(path)
    }
}

impl ConfigGet for &mut Config {
    /// Gets the value at the given path.
    fn get<P: Into<Path>>(&self, path: P) -> Option<&Literal> {
        self.map.get(path)
    }
}

impl ConfigSet for Config {
    /// Sets the value at the given path.
    fn set<P: Into<Path>, L: Into<Literal>>(&mut self, path: P, value: L) {
        self.map.set(path, value);
    }
}

impl ConfigSet for &mut Config {
    /// Sets the value at the given path.
    fn set<P: Into<Path>, L: Into<Literal>>(&mut self, path: P, value: L) {
        self.map.set(path, value);
    }
}

impl TryFrom<&str> for Config {
    type Error = toml::de::Error;
    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        toml::from_str(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config() -> Config {
        let mut config = Config::try_from(r#"
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

        assert_eq!(config.get(["ligen", "parse_all"].as_slice()), None);
        assert_eq!(config.get(["ligen", "parse-all"].as_slice()), Some(&false.into()));
        assert_eq!(config.get(["ligen", "default-name"].as_slice()), Some(&"library".into()));
    }
}

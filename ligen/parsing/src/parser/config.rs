use std::collections::HashMap;

use ligen_ir::Literal;

#[derive(Default)]
pub struct ParserConfig {
    map: HashMap<String, Literal>
}

impl ParserConfig {
    pub fn get(&self, key: &str) -> Option<&Literal> {
        self.map.get(key)
    }
}

impl<S: Into<String>, L: Into<Literal>, I: IntoIterator<Item = (S, L)>> From<I> for ParserConfig
{
    fn from(input: I) -> Self {
        let mut map = HashMap::new();
        for (key, value) in input.into_iter() {
            map.insert(key.into(), value.into());
        }
        Self { map }
    }
}

#[cfg(test)]
mod tests {
    use ligen_ir::Literal;

    use super::ParserConfig;

    #[test]
    fn parser_config() {
        let config = ParserConfig::from([
            ("parser::config::string", Literal::from("test")),
            ("parser::config::bool", Literal::from(true))
        ]);
        assert_eq!(config.get("parser::config::none"), None);
        assert_eq!(config.get("parser::config::string"), Some(&"test".into()));
        assert_eq!(config.get("parser::config::bool"), Some(&true.into()));
    }
}
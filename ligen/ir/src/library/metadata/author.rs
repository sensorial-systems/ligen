use std::fmt::Display;

use crate::prelude::*;

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Author {
    pub name: String,
    pub email: String,
}

impl Author {
    pub fn new(name: impl Into<String>, email: impl Into<String>) -> Self {
        let name = name.into();
        let email = email.into();
        Self { name, email }
    }
}

impl From<String> for Author {
    fn from(value: String) -> Self {
        Self::from(&value)
    }
}

impl From<&String> for Author {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<&str> for Author {
    fn from(value: &str) -> Self {
        let (name, email) = if let Some((name, email)) = value.split_once('<') {
            let name = name.trim();
            let email = email.strip_suffix('>').unwrap_or(&email).trim();
            (name.to_string(), email.to_string())
        } else {
            (value.to_string(), "".to_string())
        };
        Self::new(name, email)
    }
}

impl Display for Author {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.email.is_empty() {
            write!(f, "{}", self.name)
        } else {
            write!(f, "{} <{}>", self.name, self.email)
        }
    }
}
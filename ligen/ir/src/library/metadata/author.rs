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

impl Display for Author {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.email.is_empty() {
            write!(f, "\"{}\"", self.name)
        } else {
            write!(f, "\"{} <{}>\"", self.name, self.email)
        }
    }
}
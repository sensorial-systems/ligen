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

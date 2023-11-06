use ligen::parsing::parser::{ParserConfigSet, ParserConfigGet, ParserConfig};

use crate::prelude::*;

#[derive(Shrinkwrap, Clone)]
#[shrinkwrap(mutable)]
pub struct PythonParserConfig<T> {
    pub config: T
}

impl<T> From<T> for PythonParserConfig<T> {
    fn from(config: T) -> Self {
        Self { config }
    }
}

impl<T: Default + ParserConfigSet> Default for PythonParserConfig<T> {
    fn default() -> Self {
        let config = Default::default();
        let mut config = Self { config };
        config.set_class_variables_as_properties(false);
        config
    }
}

impl PythonParserConfig<ParserConfig> {
    pub fn new() -> PythonParserConfig<ParserConfig> {
        Default::default()
    }
}

impl<T> PythonParserConfig<T> {
    pub fn set_class_variables_as_properties(&mut self, value: bool)
    where T: ParserConfigSet
    {
        self.config.set("ligen::python::class_variables_as_properties", value);
    }

    pub fn get_class_variables_as_properties(&self) -> bool
    where T: ParserConfigGet
    {
        self.config
            .get("ligen::python::class_variables_as_properties")
            .and_then(|literal| literal.as_boolean())
            .cloned()
            .unwrap_or(false)
    }
}

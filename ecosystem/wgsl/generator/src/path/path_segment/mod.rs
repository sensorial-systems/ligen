use std::rc::Weak;

use ligen_transformer::prelude::*;
use ligen_idl::PathSegment;

use crate::{WgslIdentifierGenerator, WgslPathGenerator, WgslTypeGenerator};

pub struct WgslPathSegmentGenerator {
    pub identifier_generator: WgslIdentifierGenerator,
    pub type_generator: WgslTypeGenerator,
}

impl WgslPathSegmentGenerator {
    pub fn new(path_generator: Weak<WgslPathGenerator>) -> Self {
        let identifier_generator = Default::default();
        let type_generator = WgslTypeGenerator::new(path_generator);
        Self { identifier_generator, type_generator }
    }
}

impl Generator<&PathSegment, String> for WgslPathSegmentGenerator {
    fn generate(&self, path_segment: &PathSegment, config: &Config) -> Result<String> {
        let mut result = String::new();
        result.push_str(&self.identifier_generator.generate(&path_segment.identifier, config)?);
        if !path_segment.generics.types.is_empty() {
            let types: Vec<String> = path_segment.generics.types.iter().map(|generic| self.type_generator.generate(generic, config)).collect::<Result<Vec<String>>>()?;
            result.push_str(&format!("<{}>", types.join(", ")));
        }
        Ok(result)
    }
}
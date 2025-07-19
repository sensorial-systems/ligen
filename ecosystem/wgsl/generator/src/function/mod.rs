pub mod parameter;
pub use parameter::*;

use std::rc::Rc;

use ligen_transformer::prelude::*;
use ligen_idl::Function;
use crate::{WgslBlockGenerator, WgslIdentifierGenerator, WgslPathGenerator, WgslTypeGenerator};

pub struct WgslFunctionGenerator {
    pub identifier_generator: WgslIdentifierGenerator,
    pub parameter_generator: WgslParameterGenerator,
    pub block_generator: WgslBlockGenerator,
    pub type_generator: WgslTypeGenerator,
    pub path_generator: Rc<WgslPathGenerator>,
}

impl Default for WgslFunctionGenerator {
    fn default() -> Self {
        let path_generator = WgslPathGenerator::new();
        let type_generator = WgslTypeGenerator::new(Rc::downgrade(&path_generator));
        let block_generator = WgslBlockGenerator::default();
        let identifier_generator = WgslIdentifierGenerator;
        let parameter_generator = WgslParameterGenerator::default();
        Self {
            identifier_generator,
            parameter_generator,
            block_generator,
            type_generator,
            path_generator,
        }
    }
}

impl Generator<&Function, String> for WgslFunctionGenerator {
    fn generate(&self, function: &Function, config: &Config) -> Result<String> {
        let mut result = String::new();
        result.push_str(&format!("fn {}", self.identifier_generator.generate(&function.identifier, config)?));
        let parameters: Vec<String> = function.inputs.iter().map(|input| self.parameter_generator.generate(input, config)).collect::<Result<Vec<String>>>()?;
        result.push_str(&format!("({})", parameters.join(", ")));
        if let Some(output) = &function.output {
            result.push_str(&format!(" -> {}", self.type_generator.generate(output, config)?));
        }
        if let Some(body) = &function.body {
            result.push_str(&format!(" {}", self.block_generator.generate(body, config)?));
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use ligen_idl::{BinaryExpression, Block, Identifier, Parameter, PathSegment, Statement};

    use super::*;

    #[test]
    fn it_works() {
        let generator = WgslFunctionGenerator::default();
        let block = Block::from([
            Statement::return_(Some(BinaryExpression::new(Identifier::new("a"), "/", Identifier::new("b"))))
        ]);
        let function = Function::new("div", vec![Parameter::new("a", PathSegment::new("vec2", "f32")), Parameter::new("b", PathSegment::new("vec2", "f32"))], Some(PathSegment::new("vec2", "f32")), Some(block));
        let result = generator.generate(&function, &Config::default()).unwrap();
        assert_eq!(result, "fn div(a: vec2<f32>, b: vec2<f32>) -> vec2<f32> {return a / b;}");
    }
}
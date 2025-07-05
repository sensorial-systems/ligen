use std::rc::Rc;

use crate::{WgslIdentifierGenerator, WgslPathGenerator};

use ligen_transformer::prelude::*;
use ligen_ir::Expression;

pub struct WgslExpressionGenerator {
    pub path_generator: Rc<WgslPathGenerator>,
    pub identifier_generator: WgslIdentifierGenerator
}

impl Default for WgslExpressionGenerator {
    fn default() -> Self {
        let path_generator = WgslPathGenerator::new();
        let identifier_generator = Default::default();
        Self { path_generator, identifier_generator }
    }
}

impl Generator<&Expression, String> for WgslExpressionGenerator {
    fn generate(&self, expression: &Expression, config: &Config) -> Result<String> {
        let mut result = String::new();
        match expression {
            Expression::Path(path) => {
                result.push_str(&self.path_generator.generate(path, config)?);
            },
            Expression::Binary(binary_expression) => {
                let left = self.generate(&binary_expression.left, config)?;
                let right = self.generate(&binary_expression.right, config)?;
                let operator = self.identifier_generator.generate(&binary_expression.operator, config)?;
                result.push_str(&format!("{} {} {}", left, operator, right));
            },
            Expression::Parenthesized(parenthesized_expression) => {
                result.push_str(&format!("({})", self.generate(parenthesized_expression, config)?));
            },
            _ => todo!(),
        }
        Ok(result)
    }
}

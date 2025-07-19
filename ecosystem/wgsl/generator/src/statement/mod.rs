use ligen_transformer::prelude::*;
use ligen_idl::Statement;

use crate::WgslExpressionGenerator;

#[derive(Default)]
pub struct WgslStatementGenerator {
    pub expression_generator: WgslExpressionGenerator,
}

impl Generator<&Statement, String> for WgslStatementGenerator {
    fn generate(&self, statement: &Statement, config: &Config) -> Result<String> {
        let mut result = String::new();
        match statement {
            Statement::Return(return_statement) => {
                result.push_str("return");
                if let Some(value) = &return_statement.value {
                    result.push(' ');
                    result.push_str(&self.expression_generator.generate(value, config)?);
                }
            }
        }
        result.push(';');
        Ok(result)
    }
}

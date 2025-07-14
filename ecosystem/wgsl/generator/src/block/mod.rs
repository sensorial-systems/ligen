use ligen_transformer::prelude::*;
use ligen_ir::Block;
use crate::statement::WgslStatementGenerator;

#[derive(Default)]
pub struct WgslBlockGenerator {
    pub statement_generator: WgslStatementGenerator,
}

impl Generator<&Block, String> for WgslBlockGenerator {
    fn generate(&self, block: &Block, config: &Config) -> Result<String> {
        let mut result = String::new();
        result.push('{');
        for statement in &block.statements {
            result.push_str(&self.statement_generator.generate(statement, config)?);
        }
        result.push('}');
        Ok(result)
    }
}
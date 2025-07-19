use ligen_transformer::prelude::*;
use ligen_idl::Module;
use ligen_ir::Block;
use crate::WgslFunctionGenerator;

#[derive(Default)]
pub struct WgslModuleGenerator {
    pub function_generator: WgslFunctionGenerator
}

impl Generator<&Module<Block>, String> for WgslModuleGenerator {
    fn generate(&self, function: &Module<Block>, config: &Config) -> Result<String> {
        let mut result = String::new();
        for function in &function.functions {
            let function = self.function_generator.generate(function, config)?;
            result.push('\n');
            result.push_str(&function);
        }
        Ok(result)
    }
}

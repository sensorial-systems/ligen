use anchor_lang_idl_spec::Idl;
use ligen_ir::{attribute::Named, prelude::Result, Identifier, Literal, Module, Mutability, Object, Visibility};
use ligen_parser::prelude::*;

use crate::type_::TypeParser;
use crate::function::FunctionParser;

#[derive(Default)]
pub struct ModuleParser {
    type_parser: TypeParser,
    function_parser: FunctionParser,
}

impl Parser<Idl> for ModuleParser {
    type Output = Module;

    fn parse(&self, input: Idl, config: &Config) -> Result<Self::Output> {
        let attributes = Named::new("Address", input.address).into();
        let visibility = Visibility::Public;
        let identifier = Identifier::new(input.metadata.name.clone());
        let objects = input
            .constants
            .iter()
            .map(|constant| Ok(Object {
                mutability: Mutability::Constant,
                identifier: Identifier::new(constant.name.clone()),
                type_: self.type_parser.parse(constant.ty.clone(), config)?,
                literal: Literal::String(constant.value.clone()),
            }))
            .collect::<Result<Vec<_>>>()?;
        let functions = input
            .instructions
            .iter()
            .map(|instruction| self.function_parser.parse(instruction.clone(), config))
            .collect::<Result<Vec<_>>>()?;
        let types = Default::default();
        let imports = Default::default();
        let modules = Default::default();
        let interfaces = Default::default();
        let module = Module {
            attributes,
            visibility,
            identifier,
            imports,
            objects,
            functions,
            interfaces,
            modules,
            types,
        };
        Ok(module)
    }

    fn name(&self) -> &str {
        "Anchor IDL Module Parser"
    }
}
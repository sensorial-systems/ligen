
use anchor_lang_idl_spec::IdlInstruction;
use ligen_ir::{prelude::Result, Function, Identifier, Synchrony, Visibility};
use ligen_parser::prelude::*;

use crate::{doc::DocParser, parameter::ParameterParser, type_::TypeParser};

#[derive(Default)]
pub struct FunctionParser {
    doc_parser: DocParser,
    type_parser: TypeParser,
    parameter_parser: ParameterParser,
}

impl Transformer<IdlInstruction, Function> for FunctionParser {
    fn transform(&self, input: IdlInstruction, config: &Config) -> Result<Function> {
        let accounts = input
            .accounts
            .iter()
            .map(|account| self.parameter_parser.transform(account.clone(), config))
            .collect::<Result<Vec<_>>>()?;
        let args = input
            .args
            .iter()
            .map(|arg| self.parameter_parser.transform(arg.clone(), config))
            .collect::<Result<Vec<_>>>()?;
        let inputs = [accounts, args].concat();
        let attributes = self.doc_parser.transform(input.docs.clone(), config)?;
        let output = input.returns.map(|ty| self.type_parser.transform(ty.clone(), config)).transpose()?;
        let synchrony = Synchrony::Synchronous;
        let visibility = Visibility::Public;
        let identifier = Identifier::new(input.name.clone());
        let function = Function {
            attributes,
            visibility,
            synchrony,
            identifier,
            inputs,
            output,
        };
        Ok(function)
    }
}
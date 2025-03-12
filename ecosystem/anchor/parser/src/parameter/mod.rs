use anchor_lang_idl_spec::{IdlField, IdlInstructionAccountItem};
use ligen_ir::{prelude::Result, Identifier, Parameter, Type};
use ligen_parser::{Parser, ParserConfig};

use crate::{doc::DocParser, type_::TypeParser};




#[derive(Default)]
pub struct ParameterParser {
    doc_parser: DocParser,
    type_parser: TypeParser,
}

impl Parser<IdlInstructionAccountItem> for ParameterParser {
    type Output = Parameter;

    fn parse(&self, input: IdlInstructionAccountItem, config: &ParserConfig) -> Result<Self::Output> {
        match input {
            IdlInstructionAccountItem::Composite(_) => {
                todo!("Composite accounts not supported yet")
            }
            IdlInstructionAccountItem::Single(account) => {
                let attributes = self.doc_parser.parse(account.docs.clone(), config)?;
                let identifier = Identifier::new(account.name.clone());
                let type_name = if account.signer {
                    "Signer"
                } else {
                    "Account"
                };
                if !account.relations.is_empty() {
                    todo!("Relations not supported yet. Please report this issue to the Anchor IDL parser maintainers.")
                }
                if account.pda.is_some() {
                    todo!("PDA not supported yet. Please report this issue to the Anchor IDL parser maintainers.")
                }
                let type_ = if account.writable {
                    Type::mutable_reference(type_name)
                } else {
                    Type::constant_reference(type_name)
                };
                let type_ = if account.optional {
                    Type::option(type_)
                } else {
                    type_
                };
                let default_value = account.address.map(|address| address.into());
                let parameter = Parameter {
                    attributes,
                    identifier,
                    type_,
                    default_value,
                };
                Ok(parameter)
            }
        }
    }

    fn name(&self) -> &str {
        "Anchor IDL Parameter Parser"
    }
}

impl Parser<IdlField> for ParameterParser {
    type Output = Parameter;

    fn parse(&self, input: IdlField, config: &ParserConfig) -> Result<Self::Output> {
        let attributes = self.doc_parser.parse(input.docs.clone(), config)?;
        let identifier = Identifier::new(input.name.clone());
        let type_ = self.type_parser.parse(input.ty.clone(), config)?;
        let default_value = None;
        let parameter = Parameter {
            attributes,
            identifier,
            type_,
            default_value,
        };
        Ok(parameter)

    }

    fn name(&self) -> &str {
        "Anchor IDL Parameter Parser"
    }
}



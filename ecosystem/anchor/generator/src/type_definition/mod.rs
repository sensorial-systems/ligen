use anchor_lang_idl_spec::{IdlDefinedFields, IdlField, IdlSerialization, IdlTypeDef, IdlTypeDefTy};
use ligen_generator::prelude::*;
use ligen_generator::{Generator, GeneratorConfig};
use ligen_ir::KindDefinition;

use crate::AnchorTypeGenerator;

#[derive(Debug, Default)]
pub struct AnchorTypeDefinitionGenerator {
    type_generator: AnchorTypeGenerator
}

impl AnchorTypeDefinitionGenerator {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Generator<ligen_ir::TypeDefinition> for AnchorTypeDefinitionGenerator {
    type Output = anchor_lang_idl_spec::IdlTypeDef;
    fn generate(&self, input: &ligen_ir::TypeDefinition, config: &GeneratorConfig) -> Result<Self::Output> {
        let name = input.identifier.to_string();

        let docs = input.attributes.get_documentation();

        let serialization = IdlSerialization::Borsh;
        let repr = None;
        let generics = Default::default();

        let mut named_fields = vec![];
        let mut tuple_fields = vec![];

        if let KindDefinition::Structure(structure) = &input.definition {
            for field in &structure.fields {
                if let Some(identifier) = &field.identifier {
                    let name = identifier.to_string();
                    let docs = field.attributes.get_documentation();
                    let ty = self.type_generator.generate(&field.type_, config)?;

                    named_fields.push(IdlField {
                        docs,
                        name,
                        ty,
                    });
                } else {
                    let ty = self.type_generator.generate(&field.type_, config)?;
                    tuple_fields.push(ty);
                }
            }
        }
        
        let fields = if !named_fields.is_empty() {
            Some(IdlDefinedFields::Named(named_fields))
        } else if !tuple_fields.is_empty() {
            Some(IdlDefinedFields::Tuple(tuple_fields))
        } else {
            None
        };

        let idl_type = IdlTypeDef {
            name,
            docs,
                serialization,
                repr,
                generics,
                ty: IdlTypeDefTy::Struct { fields },
        };
        Ok(idl_type)
    }
}
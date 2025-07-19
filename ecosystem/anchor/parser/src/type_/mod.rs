use anchor_lang_idl_spec::{IdlArrayLen, IdlType};
use ligen_idl::{prelude::{Error, Result}, Identifier, Type};
use ligen_transformer::prelude::*;

#[derive(Default)]
pub struct TypeParser;

impl Transformer<IdlType, Type> for TypeParser {
    fn transform(&self, input: IdlType, _config: &Config) -> Result<Type> {
        match input {
            IdlType::Bool => Ok(Type::boolean()),
            IdlType::U8 => Ok(Type::u8()),
            IdlType::I8 => Ok(Type::i8()),
            IdlType::U16 => Ok(Type::u16()),
            IdlType::I16 => Ok(Type::i16()),
            IdlType::U32 => Ok(Type::u32()),
            IdlType::I32 => Ok(Type::i32()),
            IdlType::F32 => Ok(Type::f32()),
            IdlType::U64 => Ok(Type::u64()),
            IdlType::I64 => Ok(Type::i64()),
            IdlType::F64 => Ok(Type::f64()),
            IdlType::U128 => Ok(Type::u128()),
            IdlType::I128 => Ok(Type::i128()),
            IdlType::U256 => Ok(Identifier::new("U256").into()),
            IdlType::I256 => Ok(Identifier::new("I256").into()),
            IdlType::String => Ok(Type::string()),
            IdlType::Bytes => Ok(Type::slice(Type::u8())),
            IdlType::Pubkey => Ok(Identifier::new("Pubkey").into()),
            IdlType::Option(inner) => Ok(Type::option(self.transform(*inner, _config)?)),
            IdlType::Vec(inner) => Ok(Type::vector(self.transform(*inner, _config)?)),
            IdlType::Array(inner, len) => {
                let length = match len {
                    IdlArrayLen::Generic(name) => name.parse::<usize>().map_err(|e| Error::Message(e.to_string()))?,
                    IdlArrayLen::Value(value) => value,
                };
                Ok(Type::array(self.transform(*inner, _config)?, length))
            },
            IdlType::Defined { name, .. } => Ok(Identifier::new(name).into()),
            IdlType::Generic(name) => Ok(Identifier::new(name).into()),
            _ => Err(Error::Message(format!("Unsupported type: {input:?}"))),
        }
    }

    fn name(&self) -> &str {
        "Anchor IDL Type Parser"
    }
}


use anchor_lang_idl_spec::IdlType;
use ligen_generator::prelude::*;
use ligen_ir::Type;

#[derive(Debug, Default)]
pub struct AnchorTypeGenerator;

impl AnchorTypeGenerator {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Generator<&ligen_ir::Type, anchor_lang_idl_spec::IdlType> for AnchorTypeGenerator {
    fn generate(&self, type_: &ligen_ir::Type, _config: &Config) -> Result<anchor_lang_idl_spec::IdlType> {
        let result = if *type_ == Type::u8() {
            IdlType::U8
        } else if *type_ == Type::u16() {
            IdlType::U16
        } else if *type_ == Type::u32() {
            IdlType::U32
        } else if *type_ == Type::u64() {
            IdlType::U64
        } else if *type_ == Type::u128() {
            IdlType::U128
        } else if *type_ == Type::i8() {
            IdlType::I8
        } else if *type_ == Type::i16() {
            IdlType::I16
        } else if *type_ == Type::i32() {
            IdlType::I32
        } else if *type_ == Type::i64() {
            IdlType::I64
        } else if *type_ == Type::i128() {
            IdlType::I128
        } else if *type_ == Type::f32() {
            IdlType::F32
        } else if *type_ == Type::f64() {
            IdlType::F64
        } else if *type_ == Type::string() {
            IdlType::String
        } else if *type_ == Type::boolean() {
            IdlType::Bool
        } else {
            return Err(anyhow::anyhow!("Unsupported type: {}", type_).into());
        };
        Ok(result)
    }
}
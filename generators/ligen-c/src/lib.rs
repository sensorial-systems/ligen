use ligen_ir::*;

use ligen_traits::generator::file_generator::{Inputs, Template, TemplateBasedGenerator, TemplateRegister};
use std::path::PathBuf;
use std::str::FromStr;

use ligen_traits::prelude::*;
use ligen_traits::{register_functions, register_templates};

#[derive(Debug, Default)]
pub struct CGenerator;

impl TemplateRegister for CGenerator {
    fn register_templates(&self, template: &mut Template) -> Result<()> {
        register_templates!(template, identifier, arguments, implementation, method, function, module, object, parameters, project);
        Ok(())
    }
}

fn type_mapping(type_: &Type, root: bool) -> String {
    match type_ {
        Type::Reference(reference) => {
            let type_ = type_mapping(&reference.type_, false);
            match reference.mutability {
                Mutability::Mutable => format!("{}*", type_),
                Mutability::Constant => format!("const {}*", type_),
            }
        },
        Type::Compound(compound, _generics) => {
            // FIXME: Hardcoded.
            let opaque = true && root;
            let mut mapped = compound.to_string("_");
            if opaque {
                mapped.push('*');
                mapped
            } else {
                mapped
            }
        },
        Type::Primitive(primitive) => {
            match primitive {
                Primitive::Boolean => "bool",
                Primitive::Character => "char",
                Primitive::Float(float) => {
                    match float {
                        Float::F32 => "float",
                        Float::F64 => "double"
                    }
                },
                Primitive::Integer(integer) => {
                    match integer {
                        Integer::I8 => "int8_t",
                        Integer::U8 => "uint8_t",
                        Integer::I16 => "int16_t",
                        Integer::U16 => "uint16_t",
                        Integer::I32 => "int32_t",
                        Integer::U32 => "uint32_t",
                        Integer::I64 => "int64_t",
                        Integer::U64 => "uint64_t",
                        Integer::I128 => "int128_t",
                        Integer::U128 => "uint128_t",
                        Integer::ISize => "size_t",
                        Integer::USize => "usize_t"
                    }
                }
            }.to_string()
        }
    }.to_string()
}

fn mapped_type(inputs: &Inputs) -> String {
    let type_ = inputs
        .get(0)
        .and_then(|input| serde_json::from_value::<Type>(input).ok());
    if let Some(type_) = type_ {
        type_mapping(&type_, true)
    } else {
        "void".to_string()
    }
}

impl TemplateBasedGenerator for CGenerator {
    fn register_functions(&self, _project: &Project, template: &mut Template) {
        register_functions!(template, mapped_type);
    }

    fn base_path(&self) -> PathBuf {
        PathBuf::from("c".to_string())
    }

    fn module_generation_path(&self, _project: &Project, module: &Module) -> PathBuf {
        let mut path = PathBuf::from_str("include").unwrap();
        path = path.join(PathBuf::from(module.path.clone()));
        path = path.with_extension("h");
        path
    }
}
// FIXME: Move this to a generation module.
use ligen_ir::*;

use ligen_traits::generator::file_generator::{TemplateBasedGenerator, TemplateRegister, Template, Inputs};
use std::path::PathBuf;
use std::str::FromStr;
use ligen_ir::Type;

use ligen_traits::prelude::*;
use ligen_traits::{register_functions, register_templates};

fn marshal_output(inputs: &Inputs) -> String {
    let type_ = inputs
        .get(0)
        .and_then(|input| serde_json::from_value::<Type>(input).ok());
    if let Some(Type::Composite(_, _)) = type_ {
        "Box::into_raw(Box::new(result))"
    } else {
        "result"
    }.into()
}

fn type_mapping(type_: &Type, root: bool) -> String {
    match type_ {
        Type::Reference(reference) => {
            let type_ = type_mapping(&reference.type_, false);
            match reference.mutability {
                // FIXME: Change this to pointers and check if they are null or not.
                Mutability::Mutable => format!("&mut {}", type_),
                Mutability::Constant => format!("&{}", type_),
            }
        },
        Type::Composite(composite, _generics) => {
            // FIXME: Hardcoded.
            let opaque = true && root;
            let mapped = composite.to_string("::");
            if opaque {
                format!("*mut {}", mapped)
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
                        Float::F32 => "f32",
                        Float::F64 => "f64"
                    }
                },
                Primitive::Integer(integer) => {
                    match integer {
                        Integer::I8 => "i8",
                        Integer::U8 => "u8",
                        Integer::I16 => "i16",
                        Integer::U16 => "u16",
                        Integer::I32 => "i32",
                        Integer::U32 => "u32",
                        Integer::I64 => "i64",
                        Integer::U64 => "u64",
                        Integer::I128 => "i128",
                        Integer::U128 => "u128",
                        Integer::ISize => "isize",
                        Integer::USize => "usize"
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
        "()".to_string()
    }
}

#[derive(Debug, Default)]
pub struct RustGenerator;

impl TemplateRegister for RustGenerator {
    fn register_templates(&self, template: &mut Template) -> Result<()> {
        register_templates!(template, identifier, arguments, implementation, method, function, module, object, parameters, project);
        Ok(())
    }
}

impl TemplateBasedGenerator for RustGenerator {
    fn register_functions(&self, _project: &Project, template: &mut Template) {
        register_functions!(template, mapped_type, marshal_output);
    }

    fn base_path(&self) -> PathBuf {
        PathBuf::from("rust".to_string())
    }

    fn module_generation_path(&self, _project: &Project, _module: &Module) -> PathBuf {
        // let is_root_module = project.root_module == *module;
        // let name = if is_root_module { "lib.rs" } else { "ligen_editor" };
        let path = PathBuf::from_str("src").unwrap();
        // path = path.join(PathBuf::from(module.path.clone().without_first()));
        // path = path.join(name);
        // FIXME: This is not working.
        path
    }
}

use ligen::ir;

use crate::ast::Identifier;

#[derive(Debug, PartialEq, Copy, Clone)]
/// Primitive Enum
pub enum Primitive {
    /// Char variant
    Char,
    /// Bool variant
    Bool,
    /// Byte variant
    Byte,
    /// SByte variant
    SByte,
    /// Short variant
    Short,
    /// UShort variant
    UShort,
    /// Int variant
    Int,
    /// UnsignedInt variant
    UInt,
    /// Long variant
    Long,
    /// ULong variant
    ULong,
    /// NInt variant
    NInt,
    /// NUint variant
    NUInt,
    /// Float variant
    Float,
    /// Double variant
    Double,
}

impl AsRef<str> for Primitive {
    fn as_ref(&self) -> &str {
        match self {
            Primitive::Short => "short",
            Primitive::UShort => "ushort",
            Primitive::Int => "int",
            Primitive::UInt => "uint",
            Primitive::Long => "long",
            Primitive::ULong => "ulong",
            Primitive::Float => "float",
            Primitive::Double => "double",
            Primitive::NInt => "nint",
            Primitive::NUInt => "nuint",
            Primitive::Char => "char",
            Primitive::Byte => "byte",
            Primitive::SByte => "sbyte",
            Primitive::Bool => "bool"
        }
    }
}

#[derive(Debug, PartialEq)]
/// Types Enum
pub enum Types {
    /// Primitive variant
    Primitive(Primitive),
    /// Composite variant
    Composite(Identifier),
}

/// Constant.
#[derive(Debug, Clone, Copy)]
pub struct Const;

/// Pointer.
#[derive(Debug, Clone, Copy)]
pub struct Pointer;

#[derive(Debug)]
/// Type Struct
pub struct Type {
    /// constness field
    pub constness: Option<Const>,
    /// type_ field
    pub type_: Types,
    /// pointer field
    pub pointer: Option<Pointer>,
}

impl Type {
    /// Function to create a new Type
    pub fn new(constness: Option<Const>, type_: Types, pointer: Option<Pointer>) -> Type {
        Type {
            constness,
            type_,
            pointer,
        }
    }
}

impl From<ir::Primitive> for Primitive {
    fn from(primitive: ir::Primitive) -> Self {
        match primitive {
            ir::Primitive::Integer(integer) => match integer {
                ir::Integer::U8 => Primitive::Byte,
                ir::Integer::U16 => Primitive::UShort,
                ir::Integer::U32 => Primitive::UInt,
                ir::Integer::U64 => Primitive::ULong,
                ir::Integer::I8 => Primitive::SByte,
                ir::Integer::I16 => Primitive::Short,
                ir::Integer::I32 => Primitive::Int,
                ir::Integer::I64 => Primitive::Long,
                ir::Integer::U128 | ir::Integer::USize | ir::Integer::I128 | ir::Integer::ISize => {
                    panic!("Primitive types u128, usize, i128 and isize not implemented")
                }
            },
            ir::Primitive::Float(float) => match float {
                ir::Float::F32 => Primitive::Float,
                ir::Float::F64 => Primitive::Double,
            },
            ir::Primitive::Boolean => Primitive::Bool,
            ir::Primitive::Character => Primitive::Char,
        }
    }
}

impl From<ir::Type> for Types {
    fn from(type_: ir::Type) -> Self {
        match type_ {
            ir::Type::Primitive(primitive) => Self::Primitive(Primitive::from(primitive)),
            ir::Type::Composite(composite, _) => {
                Self::Composite(composite.segments.last().unwrap().clone())
            }
            ir::Type::Reference(_reference) => {
                unimplemented!("Conversion from reference to Types isn't implemented yet.")
            }
        }
    }
}

impl From<ir::Reference> for Type {
    fn from(type_: ir::Reference) -> Self {
        let constness = match type_.mutability {
            Mutability::Constant => Some(Const),
            Mutability::Mutable => None
        };
        let type_ = Types::from(*type_.type_.clone());
        let pointer = Some(Pointer);
        Self {
            constness,
            type_,
            pointer,
        }
    }
}

impl From<ir::Type> for Type {
    fn from(type_: ir::Type) -> Self {
        match type_ {
            ir::Type::Primitive(type_) => Self {
                constness: None,
                type_: Types::Primitive(type_.into()),
                pointer: None,
            },
            ir::Type::Composite(path, _) => Self {
                constness: None,
                type_: Types::Composite(path.segments.last().unwrap().clone()),
                pointer: None,
            },
            ir::Type::Reference(reference) => Self::from(reference),
        }
    }
}

use std::fmt;
use ligen::ir::Mutability;

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.pointer.is_some() {
            write!(f, "IntPtr")
        } else {
            match &self.type_ {
                Types::Primitive(primitive) => write!(f, "{}", primitive.as_ref()),
                Types::Composite(identifier) => write!(f, "{}", identifier.name)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Type;
    use ligen::ir::{Mutability, Reference, ReferenceKind};

    #[test]
    fn ast_type_primitive() {
        let out_type = ligen::ir::Type::Reference(Reference { kind: ReferenceKind::Pointer, mutability: Mutability::Constant, type_: ligen::ir::Type::Composite("i8".into(), Default::default()).into() });
        let in_type = Type::from(out_type);
        println!("{:#?}", in_type);
    }
}

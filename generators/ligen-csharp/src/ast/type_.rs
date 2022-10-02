use ligen::ir;

use crate::ast::Identifier;

#[derive(Debug, PartialEq, Copy, Clone)]
/// Atomic Enum
pub enum Atomic {
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

impl AsRef<str> for Atomic {
    fn as_ref(&self) -> &str {
        match self {
            Atomic::Short => "short",
            Atomic::UShort => "ushort",
            Atomic::Int => "int",
            Atomic::UInt => "uint",
            Atomic::Long => "long",
            Atomic::ULong => "ulong",
            Atomic::Float => "float",
            Atomic::Double => "double",
            Atomic::NInt => "nint",
            Atomic::NUInt => "nuint",
            Atomic::Char => "char",
            Atomic::Byte => "byte",
            Atomic::SByte => "sbyte",
            Atomic::Bool => "bool"
        }
    }
}

#[derive(Debug, PartialEq)]
/// Types Enum
pub enum Types {
    /// Atomic variant
    Atomic(Atomic),
    /// Compound variant
    Compound(Identifier),
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

impl From<ir::Atomic> for Atomic {
    fn from(atomic: ir::Atomic) -> Self {
        match atomic {
            ir::Atomic::Integer(integer) => match integer {
                ir::Integer::U8 => Atomic::Byte,
                ir::Integer::U16 => Atomic::UShort,
                ir::Integer::U32 => Atomic::UInt,
                ir::Integer::U64 => Atomic::ULong,
                ir::Integer::I8 => Atomic::SByte,
                ir::Integer::I16 => Atomic::Short,
                ir::Integer::I32 => Atomic::Int,
                ir::Integer::I64 => Atomic::Long,
                ir::Integer::U128 | ir::Integer::USize | ir::Integer::I128 | ir::Integer::ISize => {
                    panic!("Atomic types u128, usize, i128 and isize not implemented")
                }
            },
            ir::Atomic::Float(float) => match float {
                ir::Float::F32 => Atomic::Float,
                ir::Float::F64 => Atomic::Double,
            },
            ir::Atomic::Boolean => Atomic::Bool,
            ir::Atomic::Character => Atomic::Char,
        }
    }
}

impl From<ir::Type> for Types {
    fn from(type_: ir::Type) -> Self {
        match type_ {
            ir::Type::Atomic(atomic) => Self::Atomic(Atomic::from(atomic)),
            ir::Type::Compound(compound, _) => {
                Self::Compound(compound.segments.last().unwrap().clone())
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
            ir::Type::Atomic(type_) => Self {
                constness: None,
                type_: Types::Atomic(type_.into()),
                pointer: None,
            },
            ir::Type::Compound(path, _) => Self {
                constness: None,
                type_: Types::Compound(path.segments.last().unwrap().clone()),
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
                Types::Atomic(atomic) => write!(f, "{}", atomic.as_ref()),
                Types::Compound(identifier) => write!(f, "{}", identifier.name)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Type;
    use ligen::ir::{Mutability, Reference, ReferenceKind};

    #[test]
    fn ast_type_atomic() {
        let out_type = ligen::ir::Type::Reference(Reference { kind: ReferenceKind::Pointer, mutability: Mutability::Constant, type_: ligen::ir::Type::Compound("i8".into(), Default::default()).into() });
        let in_type = Type::from(out_type);
        println!("{:#?}", in_type);
    }
}

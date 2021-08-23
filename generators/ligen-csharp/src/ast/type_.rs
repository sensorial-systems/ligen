use ligen::ir;

use crate::ast::Identifier;

#[derive(Debug, PartialEq, Copy, Clone)]
/// Atomic Enum
pub enum Atomic {
    /// Char variant
    Char,
    /// Short variant
    Short,
    /// Int variant
    Int,
    /// LongInt variant
    LongInt,
    /// LongLongInt variant
    LongLongInt,
    /// UnsignedChar variant
    UnsignedChar,
    /// UnsignedShort variant
    UnsignedShort,
    /// UnsignedInt variant
    UnsignedInt,
    /// UnsignedLongInt variant
    UnsignedLongInt,
    /// UnsignedLongLongInt variant
    UnsignedLongLongInt,
    /// Float variant
    Float,
    /// Dobule variant
    Double,
    /// LongDouble variant
    LongDouble,
}

impl AsRef<str> for Atomic {
    fn as_ref(&self) -> &str {
        match self {
            Atomic::Char => "char",
            Atomic::Short => "short",
            Atomic::Int => "int",
            Atomic::LongInt => "long int",
            Atomic::LongLongInt => "long long int",
            Atomic::Float => "float",
            Atomic::Double => "double",
            Atomic::LongDouble => "long double",
            Atomic::UnsignedChar => "unsigned char",
            Atomic::UnsignedShort => "unsigned short",
            Atomic::UnsignedInt => "unsigned int",
            Atomic::UnsignedLongInt => "unsigned long int",
            Atomic::UnsignedLongLongInt => "unsigned long long int",
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
                ir::Integer::U8 => Atomic::UnsignedChar,
                ir::Integer::U16 => Atomic::UnsignedShort,
                ir::Integer::U32 => Atomic::UnsignedInt,
                ir::Integer::U64 => Atomic::UnsignedLongLongInt,
                ir::Integer::I8 => Atomic::Char,
                ir::Integer::I16 => Atomic::Short,
                ir::Integer::I32 => Atomic::Int,
                ir::Integer::I64 => Atomic::LongLongInt,
                ir::Integer::U128 | ir::Integer::USize | ir::Integer::I128 | ir::Integer::ISize => {
                    panic!("Atomic types u128, usize, i128 and isize not implemented")
                }
            },
            ir::Atomic::Float(float) => match float {
                ir::Float::F32 => Atomic::Float,
                ir::Float::F64 => Atomic::Double,
            },
            ir::Atomic::Boolean => panic!("Boolean not implemented"),
            ir::Atomic::Character => panic!("16bit char not implemented"),
        }
    }
}

impl From<ir::Type> for Types {
    fn from(type_: ir::Type) -> Self {
        match type_ {
            ir::Type::Atomic(atomic) => Self::Atomic(Atomic::from(atomic)),
            ir::Type::Compound(compound) => {
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
        let constness = if type_.is_constant { Some(Const) } else { None };
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
            ir::Type::Compound(path) => Self {
                constness: None,
                type_: Types::Compound(path.segments.last().unwrap().clone()),
                pointer: None,
            },
            ir::Type::Reference(reference) => Self::from(reference),
        }
    }
}

use std::fmt;
impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(_) = self.constness {
            write!(f, "const ")?;
        }

        match &self.type_ {
            Types::Atomic(atomic) => write!(f, "{}", atomic.as_ref())?,
            Types::Compound(identifier) => match identifier.name.as_str() {
                "String" => write!(f, "const char*")?,
                _ => write!(f, "C{}", identifier.name)?,
            },
        }

        if let Some(_) = self.pointer {
            write!(f, "*")?
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::{Atomic, Const, Pointer, Type, Types};

    #[test]
    fn ast_type_atomic() {
        let types: Vec<Type> = vec![
            Atomic::Char,
            Atomic::Short,
            Atomic::Int,
            Atomic::LongInt,
            Atomic::LongLongInt,
            Atomic::Float,
            Atomic::Double,
            Atomic::LongDouble,
            Atomic::UnsignedChar,
            Atomic::UnsignedShort,
            Atomic::UnsignedInt,
            Atomic::UnsignedLongInt,
            Atomic::UnsignedLongLongInt,
        ]
        .into_iter()
        .map(|atomic| Type::new(Some(Const), Types::Atomic(atomic), Some(Pointer)))
        .collect();

        let expected: Vec<String> = vec![
            "char",
            "short",
            "int",
            "long int",
            "long long int",
            "float",
            "double",
            "long double",
            "unsigned char",
            "unsigned short",
            "unsigned int",
            "unsigned long int",
            "unsigned long long int",
        ]
        .into_iter()
        .map(|ty| format!("const {}*", ty))
        .collect();

        let mut iter = types.iter().zip(expected.iter());

        while let Some((value, expected_value)) = iter.next() {
            assert_eq!(format!("{}", value), *expected_value);
        }
    }
}

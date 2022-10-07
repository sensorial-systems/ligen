use ligen::ir;

use crate::ast::Identifier;

#[derive(Debug, PartialEq, Copy, Clone)]
/// Primitive Enum
pub enum Primitive {
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

impl AsRef<str> for Primitive {
    fn as_ref(&self) -> &str {
        match self {
            Primitive::Char => "char",
            Primitive::Short => "short",
            Primitive::Int => "int",
            Primitive::LongInt => "long int",
            Primitive::LongLongInt => "long long int",
            Primitive::Float => "float",
            Primitive::Double => "double",
            Primitive::LongDouble => "long double",
            Primitive::UnsignedChar => "unsigned char",
            Primitive::UnsignedShort => "unsigned short",
            Primitive::UnsignedInt => "unsigned int",
            Primitive::UnsignedLongInt => "unsigned long int",
            Primitive::UnsignedLongLongInt => "unsigned long long int",
        }
    }
}

#[derive(Debug, PartialEq)]
/// Types Enum
pub enum Types {
    /// Primitive variant
    Primitive(Primitive),
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

impl From<ir::Primitive> for Primitive {
    fn from(primitive: ir::Primitive) -> Self {
        match primitive {
            ir::Primitive::Integer(integer) => match integer {
                ir::Integer::U8 => Primitive::UnsignedChar,
                ir::Integer::U16 => Primitive::UnsignedShort,
                ir::Integer::U32 => Primitive::UnsignedInt,
                ir::Integer::U64 => Primitive::UnsignedLongLongInt,
                ir::Integer::I8 => Primitive::Char,
                ir::Integer::I16 => Primitive::Short,
                ir::Integer::I32 => Primitive::Int,
                ir::Integer::I64 => Primitive::LongLongInt,
                ir::Integer::U128 | ir::Integer::USize | ir::Integer::I128 | ir::Integer::ISize => {
                    todo!("Primitive types u128, usize, i128 and isize not implemented")
                }
            },
            ir::Primitive::Float(float) => match float {
                ir::Float::F32 => Primitive::Float,
                ir::Float::F64 => Primitive::Double,
            },
            ir::Primitive::Boolean => todo!("Boolean not implemented"),
            ir::Primitive::Character => todo!("16bit char not implemented"),
        }
    }
}

impl From<ir::Type> for Types {
    fn from(type_: ir::Type) -> Self {
        match type_ {
            ir::Type::Primitive(primitive) => Self::Primitive(Primitive::from(primitive)),
            ir::Type::Compound(compound, _generics) => {
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
            ir::Mutability::Constant => Some(Const),
            ir::Mutability::Mutable => None
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

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(_) = self.constness {
            write!(f, "const ")?;
        }

        match &self.type_ {
            Types::Primitive(primitive) => write!(f, "{}", primitive.as_ref())?,
            Types::Compound(identifier) => match identifier.name.as_str() {
                "String" => write!(f, "const char*")?,
                _ => write!(f, "{}", identifier.name)?,
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
    use super::{Primitive, Const, Pointer, Type, Types};

    #[test]
    fn ast_type_primitive() {
        let types: Vec<Type> = vec![
            Primitive::Char,
            Primitive::Short,
            Primitive::Int,
            Primitive::LongInt,
            Primitive::LongLongInt,
            Primitive::Float,
            Primitive::Double,
            Primitive::LongDouble,
            Primitive::UnsignedChar,
            Primitive::UnsignedShort,
            Primitive::UnsignedInt,
            Primitive::UnsignedLongInt,
            Primitive::UnsignedLongLongInt,
        ]
        .into_iter()
        .map(|primitive| Type::new(Some(Const), Types::Primitive(primitive), Some(Pointer)))
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

use syn::Lit;

use syn::{AttributeArgs, ItemFn};

/// Literal Enum
#[derive(Debug)]
pub enum Literal {
    /// String variant
    String(String),
    /// Bool variant
    Bool(bool),
    /// Char variant
    Char(char),
    /// Integer variant
    Integer(i64),
    /// UnsignedInteger variant
    UnsignedInteger(u64),
    /// Float variant
    Float(f64),
}

//TODO: Implement fn as_string(&self) -> Option<&String> and variants.

impl Literal {
    //FIXME: Implement From instead
    /// Parse Literal
    pub fn parse(lit: &Lit) -> Literal {
        match lit {
            Lit::Str(litstr) => Literal::String(litstr.value()),
            Lit::Byte(litbyte) => Literal::UnsignedInteger(litbyte.value() as u64),
            Lit::Char(litchar) => Literal::UnsignedInteger(litchar.value() as u64),
            Lit::Int(litint) => Literal::Integer(litint.base10_parse().unwrap()),
            Lit::Float(litfloat) => Literal::Float(litfloat.base10_parse().unwrap()),
            Lit::Bool(litbool) => Literal::Bool(litbool.value),
            _ => Literal::String(String::from("")),
        }
    }

    //FIXME: Implement Display instead
    /// Convert Literal to String
    pub fn to_string(&self) -> String {
        match self {
            Literal::String(value) => format!("{}", value),
            Literal::Bool(value) => format!("{}", value),
            Literal::Char(value) => format!("{}", value),
            Literal::Integer(value) => format!("{}", value),
            Literal::UnsignedInteger(value) => format!("{}", value),
            Literal::Float(value) => format!("{}", value),
        }
    }
}


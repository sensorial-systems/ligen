use crate::Identifier;
use crate::Literal;
use crate::Type;

#[derive(Debug)]
pub enum Attribute {
    Literal(Literal),
    Named(Identifier, Literal),
    Group(Identifier, Attributes)
}

impl Attribute {
    pub fn parse_args(args : &syn::AttributeArgs) -> Attribute {
        if args.len() == 1 {
            Attribute::parse_nested_meta(&args[0])
        } else {
            let mut attributes = Attributes::new();

            for arg in args {
                attributes.push(Attribute::parse_nested_meta(&arg))
            }

            Attribute::Group(Identifier::new(""), attributes)
        }
    }

    pub fn parse_nested_meta(nested_meta : &syn::NestedMeta) -> Attribute {
        match &nested_meta {
            syn::NestedMeta::Meta(meta) => {
                Attribute::parse_meta(&meta)
            },
            syn::NestedMeta::Lit(lit) => {
                Attribute::Literal(Literal::parse(&lit))
            }
        }
    }

    pub fn parse_meta(meta : &syn::Meta) -> Attribute {
        match meta {
            syn::Meta::Path(path) => {
                Attribute::Group(Type::parse_path(&path).identifier, Attributes::new())
            },
            syn::Meta::List(meta_list) => {
                let mut attributes = Attributes::new();

                for nested_meta in &meta_list.nested {
                    attributes.push(Attribute::parse_nested_meta(&nested_meta))
                }

                Attribute::Group(Type::parse_path(&meta_list.path).identifier, attributes)
            },
            syn::Meta::NameValue(name_value) => {
                Attribute::Named(Type::parse_path(&name_value.path).identifier, Literal::parse(&name_value.lit))
            }
        }
    }

    pub fn parse_attribute(attribute : &syn::Attribute) -> Attribute {
        let meta = attribute.parse_meta();
        Attribute::parse_meta(&meta.unwrap())
    }

    pub fn parse_attributes(attributes : &Vec<syn::Attribute>) -> Attribute {
        if attributes.len() == 1 {
            Attribute::parse_attribute(&attributes[0])
        } else {
            let mut attributes_out = Attributes::new();

            for attribute in attributes {
                attributes_out.push(Attribute::parse_attribute(&attribute))
            }

            Attribute::Group(Identifier::new(""), attributes_out)
        }
    }
}

#[derive(Debug)]
pub struct Attributes {
    pub attributes : Vec<Attribute>
}

impl Attributes {
    pub fn new() -> Self {
        Self {
            attributes : Vec::new()
        }
    }

    pub fn from_vec(attributes : Vec<Attribute>) -> Self {
        Self {
            attributes
        }
    }

    pub fn push(&mut self, attribute : Attribute) {
        self.attributes.push(attribute)
    }

    pub fn get_named(&self, name : &str) -> Option<&Literal> {
        for attribute in &self.attributes {
            match attribute {
                Attribute::Named(identifier, lit) => {
                    if identifier.name == name {
                        return Some(&lit)
                    }
                }
                _ => ()
            }
        }
        None
    }
}

pub trait LiteralConverter {
    fn as_bool(&self, default: bool) -> bool;
    fn as_string(&self, default: &str) -> String;
    fn as_char(&self, default: char) -> char;
    fn as_integer(&self, default: i64) -> i64;
    fn as_unsigned_integer(&self, default: u64) -> u64;
    fn as_float(&self, default: f64) -> f64;
}

impl LiteralConverter for Option<&Literal> {
    fn as_bool(&self, default: bool) -> bool {
        if let Some(literal) = self {
            if let Literal::Bool(value) = literal {
                *value
            } else { default }
        } else { default }
    }

    fn as_string(&self, default: &str) -> String {
        if let Some(literal) = self {
            if let Literal::String(value) = literal {
                value.clone()
            } else { String::from(default) }
        } else { String::from(default) }
    }

    fn as_char(&self, default: char) -> char {
        if let Some(literal) = self {
            if let Literal::Char(value) = literal {
                *value
            } else { default }
        } else { default }
    }

    fn as_integer(&self, default: i64) -> i64 {
        if let Some(literal) = self {
            if let Literal::Integer(value) = literal {
                *value
            } else { default }
        } else { default }
    }

    fn as_unsigned_integer(&self, default: u64) -> u64 {
        if let Some(literal) = self {
            if let Literal::UnsignedInteger(value) = literal {
                *value
            } else { default }
        } else { default }
    }

    fn as_float(&self, default: f64) -> f64 {
        if let Some(literal) = self {
            if let Literal::Float(value) = literal {
                *value
            } else { default }
        } else { default }
    }
}


impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Attribute::Literal(literal) => write!(f, "\"{}\"", literal.to_string()),
            Attribute::Named(identifier, literal) => write!(f, "{} = \"{}\"", identifier.name, literal.to_string()),
            Attribute::Group(identifier, group) => {
                if group.attributes.len() > 0 {
                    write!(f, "{}({})", identifier.name, group)
                } else {
                    write!(f, "{}", identifier.name)
                }
            }
        }
    }
}

use std::fmt;
impl fmt::Display for Attributes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = write!(f, "");
        for (i, attribute) in self.attributes.iter().enumerate() {
            let comma = if i == self.attributes.len() - 1 { "" } else { ", " };
            result = write!(f, "{}{}", attribute, comma);
        }
        result
    }
}

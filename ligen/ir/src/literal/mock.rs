use crate::*;

pub fn literal_verbatim() -> Literal {
    Literal::String("\"verbatim\"".into())
}

pub fn literal_string() -> Literal {
    Literal::String("string".into())
}

pub fn literal_byte() -> Literal {
    Literal::UnsignedInteger(b'A' as u64)
}

pub fn literal_bool() -> Literal {
    Literal::Boolean(false)
}

pub fn literal_character() -> Literal {
    Literal::Character('A')
}

pub fn literal_integer() -> Literal {
    Literal::Integer(-2)
}

pub fn literal_float() -> Literal {
    Literal::Float(3.5)
}

pub fn literal_unknown() -> Literal {
    Literal::Unknown(".0".into())
}
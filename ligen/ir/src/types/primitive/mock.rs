use crate::*;

pub fn character() -> Primitive {
    Primitive::Character
}

pub fn boolean() -> Primitive {
    Primitive::Boolean
}

pub fn float32() -> Primitive {
    Primitive::Float(Float::F32)
}

pub fn float64() -> Primitive {
    Primitive::Float(Float::F64)
}
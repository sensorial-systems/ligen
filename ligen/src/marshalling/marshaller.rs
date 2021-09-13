use crate::ir::Type;
use std::collections::HashMap;
// TODO: Remove this if it isn't used.
// pub trait MarshallFrom<T>: Sized {
//     fn marshal_from(from: T) -> Self;
// }
//
// pub trait MarshallInto<T>: Sized {
//     fn marshal_into(self) -> T;
// }
//
// impl<T, U: MarshallFrom<T>> MarshallInto<U> for T {
//     fn marshal_into(self) -> U {
//         U::marshal_from(self)
//     }
// }

/// Marshaller.
#[derive(Debug)]
pub struct Marshaller {
    map_input: HashMap<Type, Type>,
    map_output: HashMap<Type, Type>
}

impl Marshaller {
    /// Creates a new instance of the Marshaller.
    pub fn new() -> Self {
        let map_into = HashMap::default();
        let map_from = HashMap::default();
        Self { map_input: map_into, map_output: map_from }
    }

    /// Add type mapping.
    pub fn add_input_marshalling(&mut self, from: Type, into: Type) {
        self.map_input.insert(from, into);
    }

    /// Add type mapping.
    pub fn add_output_marshalling(&mut self, from: Type, into: Type) {
        self.map_output.insert(from, into);
    }

    /// Marshal input.
    pub fn mashal_input(&self, type_: &Type) -> Type {
        let type_ = if let Some(type_) = self.map_input.get(type_) {
            type_
        } else {
            type_
        };
        type_.clone()
    }

    /// Marshal output.
    pub fn marshal_output(&self, type_: &Type) -> Type {
        let type_ = if let Some(type_) = self.map_output.get(type_) {
            type_
        } else {
            type_
        };
        type_.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{Atomic, Integer, Reference, ReferenceKind};

    #[test]
    fn atomic_to() {
        let marshaller = Marshaller::new();
        let type_ = Type::Atomic(Atomic::Integer(Integer::I32));
        assert_eq!(marshaller.mashal_input(&type_).to_string(), "i32");
    }

    #[test]
    fn reference_to() {
        let marshaller = Marshaller::new();
        let type_ = Type::Compound("Object".into());
        let type_ = Type::Reference(Reference { is_constant: false, kind: ReferenceKind::Borrow, type_: type_.into() });
        assert_eq!(marshaller.mashal_input(&type_).to_string(), "&mut Object");

        let type_ = Type::Compound("Object".into());
        let type_ = Type::Reference(Reference { is_constant: true, kind: ReferenceKind::Pointer, type_: type_.into() });
        assert_eq!(marshaller.mashal_input(&type_).to_string(), "*const Object");
    }

    #[test]
    fn compound_to() {
        let marshaller = Marshaller::new();
        let type_ = Type::Compound("Object".into());
        assert_eq!(marshaller.mashal_input(&type_).to_string(), "Object");
    }

    #[test]
    fn mapped_to() {
        let mut marshaller = Marshaller::new();
        marshaller.add_input_marshalling(Type::Compound("String".into()), Type::Reference(Reference { type_: Type::Compound("FFIString".into()).into(), kind: ReferenceKind::Pointer, is_constant: false }));
        let type_ = Type::Compound("String".into());
        assert_eq!(marshaller.mashal_input(&type_).to_string(), "*mut FFIString");
    }

    #[test]
    fn mapped_from() {
        let mut marshaller = Marshaller::new();
        marshaller.add_output_marshalling(Type::Compound("String".into()), Type::Compound("CharPointer".into()));
        let type_ = Type::Compound("String".into());
        assert_eq!(marshaller.marshal_output(&type_).to_string(), "CharPointer");
    }
}
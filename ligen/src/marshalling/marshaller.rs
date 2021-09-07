use crate::ir::Type;
use std::collections::HashMap;

/// Marshaller.
#[derive(Debug)]
pub struct Marshaller {
    map_into: HashMap<Type, Type>,
    map_from: HashMap<Type, Type>
}

impl Marshaller {
    /// Creates a new instance of the Marshaller.
    pub fn new() -> Self {
        let map_into = HashMap::default();
        let map_from = HashMap::default();
        Self { map_into, map_from }
    }

    /// Add type mapping.
    pub fn add_mapping_into(&mut self, from: Type, into: Type) {
        self.map_into.insert(from, into);
    }

    /// Add type mapping.
    pub fn add_mapping_from(&mut self, from: Type, into: Type) {
        self.map_from.insert(from, into);
    }

    /// Marshal to.
    pub fn marshal_to(&self, type_: &Type) -> Type {
        let type_ = if let Some(type_) = self.map_into.get(type_) {
            type_
        } else {
            type_
        };
        type_.clone()
    }

    /// Marshal from.
    pub fn marshal_from(&self, type_: &Type) -> Type {
        let type_ = if let Some(type_) = self.map_from.get(type_) {
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
        assert_eq!(marshaller.marshal_to(&type_).to_string(), "i32");
    }

    #[test]
    fn reference_to() {
        let marshaller = Marshaller::new();
        let type_ = Type::Compound("Object".into());
        let type_ = Type::Reference(Reference { is_constant: false, kind: ReferenceKind::Borrow, type_: type_.into() });
        assert_eq!(marshaller.marshal_to(&type_).to_string(), "&mut Object");

        let type_ = Type::Compound("Object".into());
        let type_ = Type::Reference(Reference { is_constant: true, kind: ReferenceKind::Pointer, type_: type_.into() });
        assert_eq!(marshaller.marshal_to(&type_).to_string(), "*const Object");
    }

    #[test]
    fn compound_to() {
        let marshaller = Marshaller::new();
        let type_ = Type::Compound("Object".into());
        assert_eq!(marshaller.marshal_to(&type_).to_string(), "Object");
    }

    #[test]
    fn mapped_to() {
        let mut marshaller = Marshaller::new();
        marshaller.add_mapping_into(Type::Compound("String".into()), Type::Reference(Reference { type_: Type::Compound("FFIString".into()).into(), kind: ReferenceKind::Pointer, is_constant: false }));
        let type_ = Type::Compound("String".into());
        assert_eq!(marshaller.marshal_to(&type_).to_string(), "*mut FFIString");
    }

    #[test]
    fn mapped_from() {
        let mut marshaller = Marshaller::new();
        marshaller.add_mapping_from(Type::Compound("String".into()), Type::Compound("CharPointer".into()));
        let type_ = Type::Compound("String".into());
        assert_eq!(marshaller.marshal_from(&type_).to_string(), "CharPointer");
    }
}
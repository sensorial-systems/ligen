use crate::ir::{Type, TypeDefinition, Attribute, Reference, ReferenceKind};
use crate::generator::{ModuleVisitor, ProjectVisitor, StructureVisitor};
use std::collections::HashMap;

/// Marshal type from.
pub trait MarshalFrom<T>: Sized {
    /// Performs the marshalling.
    fn marshal_from(from: T) -> Self;
}

/// Marshal type into.
pub trait MarshalInto<T>: Sized {
    /// Performs the marshalling.
    fn marshal_into(self) -> T;
}

impl<T, U: MarshalFrom<T>> MarshalInto<U> for T {
    fn marshal_into(self) -> U {
        U::marshal_from(self)
    }
}

impl<T> MarshalFrom<T> for T {
    fn marshal_from(from: Self) -> Self {
        from
    }
}

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

    /// Register marshallers in project.
    pub fn register_project(&mut self, project: &ProjectVisitor) {
        let module = ModuleVisitor::from(&project.child(project.root_module.clone()));
        self.register_module(&module);
    }

    /// Register marshallers in module.
    pub fn register_module(&mut self, module: &ModuleVisitor) {
        for child_module in &module.current.modules {
            let module = ModuleVisitor::from(&module.child(child_module.clone()));
            self.register_module(&module);
        }
        for object in &module.current.objects {
            let object = module.child(object.clone());
            match &object.current.definition {
                TypeDefinition::Structure(structure) => self.register_structure(&object.child(structure.clone())),
                _ => ()
            }
        }
    }

    /// Register masrhallers in definition.
    pub fn register_structure(&mut self, structure: &StructureVisitor) {
        if structure.current.attributes.contains(&Attribute::Group("ligen".into(), Attribute::Group("opaque".into(), Default::default()).into())) {
            println!("Path: {}", structure.path());
            let type_ = Type::from(structure.path());
            let opaque_type = Type::Reference(Reference {
                kind: ReferenceKind::Pointer,
                is_constant: false,
                type_: type_.clone().into()
            });
            self.add_input_marshalling(type_.clone(), opaque_type.clone());
            self.add_output_marshalling(type_, opaque_type);
        }
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
    pub fn marshal_input(&self, type_: &Type) -> Type {
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

    struct A;
    struct B;

    impl MarshalFrom<A> for B {
        fn marshal_from(_a: A) -> Self {
            B
        }
    }

    #[test]
    fn marshal_trait() {
        B::marshal_from(A);
        B::marshal_from(B);
    }

    #[test]
    fn atomic_to() {
        let marshaller = Marshaller::new();
        let type_ = Type::Atomic(Atomic::Integer(Integer::I32));
        assert_eq!(marshaller.marshal_input(&type_).to_string(), "i32");
    }

    #[test]
    fn reference_to() {
        let marshaller = Marshaller::new();
        let type_ = Type::Compound("Object".into());
        let type_ = Type::Reference(Reference { is_constant: false, kind: ReferenceKind::Borrow, type_: type_.into() });
        assert_eq!(marshaller.marshal_input(&type_).to_string(), "&mut Object");

        let type_ = Type::Compound("Object".into());
        let type_ = Type::Reference(Reference { is_constant: true, kind: ReferenceKind::Pointer, type_: type_.into() });
        assert_eq!(marshaller.marshal_input(&type_).to_string(), "*const Object");
    }

    #[test]
    fn compound_to() {
        let marshaller = Marshaller::new();
        let type_ = Type::Compound("Object".into());
        assert_eq!(marshaller.marshal_input(&type_).to_string(), "Object");
    }

    #[test]
    fn mapped_to() {
        let mut marshaller = Marshaller::new();
        marshaller.add_input_marshalling(Type::Compound("String".into()), Type::Reference(Reference { type_: Type::Compound("FFIString".into()).into(), kind: ReferenceKind::Pointer, is_constant: false }));
        let type_ = Type::Compound("String".into());
        assert_eq!(marshaller.marshal_input(&type_).to_string(), "*mut FFIString");
    }

    #[test]
    fn mapped_from() {
        let mut marshaller = Marshaller::new();
        marshaller.add_output_marshalling(Type::Compound("String".into()), Type::Compound("CharPointer".into()));
        let type_ = Type::Compound("String".into());
        assert_eq!(marshaller.marshal_output(&type_).to_string(), "CharPointer");
    }
}
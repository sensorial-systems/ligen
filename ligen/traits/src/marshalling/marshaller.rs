// TODO: Remove this file if it is no longer used.
// use ligen_idl::{Type, Attribute, Reference, Mutability};
// use ligen_utils::visitors::{ModuleVisitor, LibraryVisitor, StructureVisitor};
// use std::collections::HashMap;
//
// /// Marshal type from.
// pub trait MarshalFrom<T>: Sized {
//     /// Performs the marshalling.
//     fn marshal_from(from: T) -> Self;
// }
//
// /// Marshal type into.
// pub trait MarshalInto<T>: Sized {
//     /// Performs the marshalling.
//     fn marshal_into(self) -> T;
// }
//
// impl<T, U: MarshalFrom<T>> MarshalInto<U> for T {
//     fn marshal_into(self) -> U {
//         U::marshal_from(self)
//     }
// }
//
// impl<T> MarshalFrom<T> for T {
//     fn marshal_from(from: Self) -> Self {
//         from
//     }
// }
//
// /// Marshaller.
// #[derive(Debug, Default)]
// pub struct Marshaller {
//     map_input: HashMap<Type, Type>,
//     map_output: HashMap<Type, Type>
// }
//
// impl Marshaller {
//     /// Creates a new instance of the Marshaller.
//     pub fn new() -> Self {
//         Default::default()
//     }
//
//     /// Register marshallers in library.
//     pub fn register_library(&mut self, library: &LibraryVisitor) {
//         let module = ModuleVisitor::from(&library.child(library.root_module.clone()));
//         self.register_module(&module);
//     }
//
//     /// Register marshallers in module.
//     pub fn register_module(&mut self, module: &ModuleVisitor) {
//         for child_module in &module.current.modules {
//             let module = ModuleVisitor::from(&module.child(child_module.clone()));
//             self.register_module(&module);
//         }
//         for object in &module.current.objects {
//             let object = module.child(object.clone());
//             if let TypeDefinition::Structure(structure) = &object.current.definition {
//                 self.register_structure(&object.child(structure.clone()));
//             }
//         }
//     }
//
//     /// Register masrhallers in definition.
//     pub fn register_structure(&mut self, structure: &StructureVisitor) {
//         if structure.parent.attributes.contains(&Attribute::Group("ligen".into(), Attribute::Group("opaque".into(), Default::default()).into())) {
//             let type_ = Type::from(structure.path());
//             let opaque_type = Type::Reference(Reference {
//                 mutability: Mutability::Constant,
//                 type_: type_.clone().into()
//             });
//             self.add_input_marshalling(type_.clone(), opaque_type.clone());
//             self.add_output_marshalling(type_, opaque_type);
//         }
//     }
//
//     /// Add type mapping.
//     pub fn add_input_marshalling(&mut self, from: Type, into: Type) {
//         self.map_input.insert(from, into);
//     }
//
//     /// Add type mapping.
//     pub fn add_output_marshalling(&mut self, from: Type, into: Type) {
//         self.map_output.insert(from, into);
//     }
//
//     /// Marshal input.
//     pub fn marshal_input(&self, type_: &Type) -> Type {
//         let type_ = if let Some(type_) = self.map_input.get(type_) {
//             type_
//         } else {
//             type_
//         };
//         type_.clone()
//     }
//
//     /// Marshal output.
//     pub fn marshal_output(&self, type_: &Type) -> Type {
//         let type_ = if let Some(type_) = self.map_output.get(type_) {
//             type_
//         } else {
//             type_
//         };
//         type_.clone()
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use ligen_idl::{Primitive, Integer, Reference};
//
//     struct A;
//     struct B;
//
//     impl MarshalFrom<A> for B {
//         fn marshal_from(_a: A) -> Self {
//             B
//         }
//     }
//
//     #[test]
//     fn marshal_trait() {
//         B::marshal_from(A);
//         B::marshal_from(B);
//     }
//
//     #[test]
//     fn primitive_to() {
//         let marshaller = Marshaller::new();
//         let type_ = Type::Primitive(Primitive::Integer(Integer::I32));
//         assert_eq!(marshaller.marshal_input(&type_).to_string(), "i32");
//     }
//
//     #[test]
//     fn reference_to() {
//         let marshaller = Marshaller::new();
//         let type_ = Type::Composite("Object".into(), Default::default());
//         let type_ = Type::Reference(Reference { mutability: Mutability::Constant, type_: type_.into() });
//         assert_eq!(marshaller.marshal_input(&type_).to_string(), "*const Object");
//
//         let type_ = Type::Composite("Object".into(), Default::default());
//         let type_ = Type::Reference(Reference { mutability: Mutability::Mutable, type_: type_.into() });
//         assert_eq!(marshaller.marshal_input(&type_).to_string(), "*mut Object");
//     }
//
//     #[test]
//     fn composite_to() {
//         let marshaller = Marshaller::new();
//         let type_ = Type::Composite("Object".into(), Default::default());
//         assert_eq!(marshaller.marshal_input(&type_).to_string(), "Object");
//     }
//
//     #[test]
//     fn mapped_to() {
//         let mut marshaller = Marshaller::new();
//         marshaller.add_input_marshalling(Type::Composite("String".into(), Default::default()), Type::Reference(Reference { type_: Type::Composite("FFIString".into(), Default::default()).into(), mutability: Mutability::Constant }));
//         let type_ = Type::Composite("String".into(), Default::default());
//         let marshalled_type = marshaller.marshal_input(&type_);
//         assert_eq!(marshalled_type.to_string(), "*const FFIString");
//     }
//
//     #[test]
//     fn mapped_from() {
//         let mut marshaller = Marshaller::new();
//         marshaller.add_output_marshalling(Type::Composite("String".into(), Default::default()), Type::Composite("CharPointer".into(), Default::default()));
//         let type_ = Type::Composite("String".into(), Default::default());
//         assert_eq!(marshaller.marshal_output(&type_).to_string(), "CharPointer");
//     }
// }

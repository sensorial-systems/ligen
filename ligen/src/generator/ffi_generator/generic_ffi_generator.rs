use crate::generator::{File, ProjectVisitor, FunctionVisitor, ImplementationVisitor, ModuleVisitor, ObjectVisitor, FFIGenerator};
use crate::ir::{Identifier, ImplementationItem, Type, Visibility};
use crate::ir::processing::ReplaceIdentifier;
use crate::marshalling::Marshaller;

/// A generic FFI generator which can be used for most languages.
pub trait GenericFFIGenerator {
    /// Generate the function parameters.
    fn generate_parameters(marshaller: &Marshaller, file: &mut File, visitor: &FunctionVisitor) {
        let object_identifier = visitor.parent.current.self_.path().last();
        for parameter in &visitor.current.inputs {
            let type_ = marshaller.marshal_to(&parameter.type_);
            let identifier = self_to_explicit_name(&parameter.identifier, &object_identifier);
            file.write(format!("{identifier}: {type_}, ", identifier = identifier, type_ = type_))
        }
    }

    /// Generate the function call arguments and its conversions.
    fn generate_arguments(file: &mut File, function: &FunctionVisitor) {
        let object_identifier = function.parent.current.self_.path().last();
        for input in &function.current.inputs {
            let identifier = self_to_explicit_name(&input.identifier, &object_identifier);
            file.write(format!("{identifier}.into(), ", identifier = identifier));
        }
    }

    /// Generate the function output.
    fn generate_output(marshaller: &Marshaller, file: &mut File, output: &Option<Type>) {
        match output {
            Some(type_) => file.write(&format!(" -> {}", marshaller.marshal_to(&type_))),
            _ => ()
        }
    }

    /// Generate the function
    fn generate_function_signature(marshaller: &Marshaller, file: &mut File, visitor: &FunctionVisitor) {
        let implementation = &visitor.parent.current;
        let function = &visitor.current;
        let function_name = format!("{}_{}", implementation.self_.path().last().name, function.identifier.name);
        let function_identifier = Identifier::new(&function_name);
        file.writeln("#[no_mangle]");
        file.write(format!("pub extern fn {function_identifier}(", function_identifier = function_identifier));
        Self::generate_parameters(marshaller, file, visitor);
        file.write(")");
        Self::generate_output(marshaller, file, &function.output);
    }

    /// Generate the function
    fn generate_function_block(_marshaller: &Marshaller, file: &mut File, visitor: &FunctionVisitor) {
        let method = &visitor.current;
        let implementation = &visitor.parent.current;
        let self_identifier = &implementation.self_;
        let method_identifier = &method.identifier;
        // let result = if let Some(type_) = method.output.as_ref() {
        //     let output_type = marshaller.marshal_from(type_);
        //     if let Type::Reference(_) = output_type {
        //         "Box::into_raw(Box::new(result.into()))".to_string()
        //     } else {
        //         "result".to_string()
        //     }
        // } else {
        //     "result".to_string()
        // };
        file.writeln(" {");
        file.write(format!("\tlet result = {}::{}(", self_identifier, method_identifier));
        Self::generate_arguments(file, visitor);
        file.writeln(");");
        file.writeln("\tresult.into()");
        file.writeln("}");
    }

    /// Generate an extern function for an implementation method.
    fn generate_function(marshaller: &Marshaller, file: &mut File, visitor: &FunctionVisitor) {
        if let Visibility::Public = visitor.current.visibility {
            Self::generate_function_signature(marshaller, file, visitor);
            Self::generate_function_block(marshaller, file, visitor);
        }
    }

    /// Generate drop extern.
    fn generate_drop(file: &mut File, visitor: &ObjectVisitor) {
        let self_path = &visitor.current.path;
        let object_name = self_path.last();
        let drop_name = Identifier::new(format!("{}_drop", object_name.name).as_str());
        file.writeln("#[no_mangle]");
        file.writeln(format!("pub unsafe extern fn {}(object: *mut {}) {{", drop_name, object_name));
        file.writeln("\tBox::from_raw(object);");
        file.writeln("}");
    }

    /// Generate project externs.
    fn generate(marshaller: &Marshaller, file: &mut File, visitor: &ProjectVisitor) {
        Self::generate_module(marshaller, file, &visitor.child(visitor.current.root_module.clone()));
    }

    /// Generate module externs.
    fn generate_module<V: Into<ModuleVisitor>>(marshaller: &Marshaller, file: &mut File, visitor: V) {
        let visitor = &visitor.into();
        // FIXME: How to implement Join<Separator> so we can reduce verbosity?
        file.writeln(format!("use {}::*;", visitor.path().segments.iter().map(|x| x.name.clone()).collect::<Vec<_>>().join("::")));
        file.writeln("");
        for module in &visitor.current.modules {
            Self::generate_module(marshaller, file, &visitor.child(module.clone()));
        }
        for object in &visitor.current.objects {
            Self::generate_object(marshaller, file, &visitor.child(object.clone()));
        }
    }

    /// Generate object externs.
    fn generate_object(marshaller: &Marshaller, file: &mut File, visitor: &ObjectVisitor) {
        for implementation in &visitor.current.implementations {
            Self::generate_implementation(marshaller, file, &visitor.child(implementation.clone()));
        }
        Self::generate_drop(file, visitor);
    }

    /// Generate externs for Constants and Methods.
    fn generate_implementation(marshaller: &Marshaller, file: &mut File, visitor: &ImplementationVisitor) {
        for item in &visitor.current.items {
            match item {
                ImplementationItem::Constant(_) => (),
                ImplementationItem::Method(method) => Self::generate_function(marshaller, file, &visitor.child(method.clone())),
            }
        }
    }
}

fn self_to_explicit_name(identifier: &Identifier, name_identifier: &Identifier) -> Identifier {
    let mut identifier = identifier.clone();
    identifier.replace_identifier(&Identifier::new("self"), &Identifier::new(name_identifier.name.to_lowercase()));
    identifier
}

impl<T: GenericFFIGenerator> FFIGenerator for T {
    fn generate_ffi(&self, marshaller: &Marshaller, file: &mut File, visitor: &ProjectVisitor) {
        Self::generate(marshaller, file, visitor);
    }
}

use ligen::generator::{ImplementationVisitor, FileProcessorVisitor, FileSet, FunctionVisitor, ParameterVisitor, FileGeneratorVisitors, StructureVisitor, ObjectVisitor, ModuleVisitor, ProjectVisitor, EnumerationVisitor, FunctionParent};
use ligen::ir;
use std::path::PathBuf;
use crate::ast::{Types, Type};
use crate::generator::CGenerator;
use ligen::ir::Path;

/// Project processor.
#[derive(Default, Clone, Copy, Debug)]
pub struct ProjectProcessor;

/// Module processor.
#[derive(Default, Clone, Copy, Debug)]
pub struct ModuleProcessor;

/// Object processor.
#[derive(Default, Clone, Copy, Debug)]
pub struct ObjectProcessor;

/// Structure processor.
#[derive(Default, Clone, Copy, Debug)]
pub struct StructureProcessor;

#[derive(Default, Clone, Copy, Debug)]
pub struct EnumerationProcessor;

/// Implementation processor.
#[derive(Default, Clone, Copy, Debug)]
pub struct ImplementationProcessor;

/// Function processor.
#[derive(Default, Clone, Copy, Debug)]
pub struct FunctionProcessor;

/// Parameter processor.
#[derive(Default, Clone, Copy, Debug)]
pub struct ParameterProcessor;

fn path(in_path: &Path) -> PathBuf {
    let mut path = PathBuf::from("include");
    for segment in &in_path.segments {
        path = path.join(segment.to_string());
    }
    path.with_extension("h")
}

impl FileProcessorVisitor for ProjectProcessor {
    type Visitor = ProjectVisitor;

    fn process(&self, _file_set: &mut FileSet, _visitor: &Self::Visitor) {}

    fn post_process(&self, _file_set: &mut FileSet, _visitor: &Self::Visitor) {}
}

impl FileProcessorVisitor for ModuleProcessor {
    type Visitor = ModuleVisitor;

    fn process(&self, _file_set: &mut FileSet, _visitor: &Self::Visitor) {}

    fn post_process(&self, _file_set: &mut FileSet, _visitor: &Self::Visitor) {}
}

impl FileProcessorVisitor for ObjectProcessor {
    type Visitor = ObjectVisitor;

    fn process(&self, file_set: &mut FileSet, object: &Self::Visitor) {
        let file = file_set.entry(&path(&object.path()));
        // includes
        file.writeln("#pragma once");
        file.writeln("");
        file.writeln("#include <stdint.h>");
        file.writeln("");
        file.writeln("#ifdef __cplusplus");
        file.writeln("extern \"C\" {");
        file.writeln("#endif\n");
    }

    fn post_process(&self, file_set: &mut FileSet, object: &Self::Visitor) {
        let file = file_set.entry(&path(&object.path()));

        // drop function
        let object_name = &object.path.last().name;
        let c_type      = Type::from(ir::Type::Compound(object.path.clone()));
        file.writeln(format!("void {name}_drop({type_} self);", name = object_name, type_ = c_type));

        // epilogue
        file.writeln("");
        file.writeln("#ifdef __cplusplus");
        file.writeln("}");
        file.writeln("#endif");
    }
}

impl FileProcessorVisitor for EnumerationProcessor {
    type Visitor = EnumerationVisitor;

    fn process(&self, _file_set: &mut FileSet, _enumeration: &Self::Visitor) {
    }

    fn post_process(&self, _file_set: &mut FileSet, _enumeration: &Self::Visitor) {}
}

impl FileProcessorVisitor for StructureProcessor {
    type Visitor = StructureVisitor;

    fn process(&self, file_set: &mut FileSet, structure: &Self::Visitor) {
        let file = file_set.entry(&path(&structure.path()));
        file.writeln(format!("typedef struct Struct_{} {{", structure.identifier));
        file.writeln("\tvoid* self;");
        file.writeln(format!("}} {};", structure.identifier));
    }

    fn post_process(&self, _file_set: &mut FileSet, _visitor: &Self::Visitor) {}
}

impl FileProcessorVisitor for ImplementationProcessor {
    type Visitor = ImplementationVisitor;

    fn process(&self, _file_set: &mut FileSet, _visitor: &Self::Visitor) {}

    fn post_process(&self, _file_set: &mut FileSet, _visitor: &Self::Visitor) {}
}

impl FunctionProcessor {
    /// Generate function name.
    pub fn generate_function_name(&self, function: &FunctionVisitor) -> String {
        // FIXME: This naming convention happens in the extern generator and here. How can we generalize this code?
        match &function.parent {
            FunctionParent::Implementation(implementation) =>
                format!("{}_{}", &implementation.self_.path().last().name, &function.identifier.name),
            FunctionParent::Module(_) =>
                format!("{}", function.identifier.name)

        }
    }

    /// Generate function output.
    pub fn generate_function_output(&self, output: &Option<ir::Type>) -> String {
        let type_ = output
            .as_ref()
            .map(|type_| {
                let typ_ = Type::from(type_.clone());
                if let Types::Compound(compound) = typ_.type_ {
                    match compound.name.as_str() {
                        // FIXME: C prefix should be generalized like in Type::From
                        "String" => "RString".to_string(),
                        _ => Type::from(type_.clone()).to_string(),
                    }
                } else {
                    Type::from(type_.clone()).to_string()
                }
            })
            .unwrap_or_else(|| "void".into());
        format!("{} ", type_)
    }
}

impl FileProcessorVisitor for FunctionProcessor {
    type Visitor = FunctionVisitor;

    fn process(&self, file_set: &mut FileSet, function: &Self::Visitor) {
        if let ir::Visibility::Public = function.visibility {
            let file = file_set.entry(&path(&function.path()));
            file.write(self.generate_function_output(&function.output));
            file.write(self.generate_function_name(&function));
            file.write("(");
        }
    }

    fn post_process(&self, file_set: &mut FileSet, function: &Self::Visitor) {
        if let ir::Visibility::Public = function.visibility {
            let file = file_set.entry(&path(&function.path()));
            file.writeln(");");
        }
    }
}

impl FileProcessorVisitor for ParameterProcessor {
    type Visitor = ParameterVisitor;

    fn process(&self, file_set: &mut FileSet, parameter: &Self::Visitor) {
        let file = file_set.entry(&path(&parameter.parent.path()));

        let mut type_ = Type::from(parameter.type_.clone());
        if let (Some(_pointer), Types::Compound(_type)) = (&type_.pointer, &type_.type_) {
            type_.pointer = None;
        }
        let ident = &parameter.identifier.name;
        file.write(format!("{} {}", type_, ident))
    }

    fn post_process(&self, file_set: &mut FileSet, visitor: &Self::Visitor) {
        let file = file_set.entry(&path(&visitor.parent.path()));
        file.write(", ");
    }
}

impl FileGeneratorVisitors for CGenerator {
    type ProjectProcessor = ProjectProcessor;
    type ModuleProcessor = ModuleProcessor;
    type ObjectProcessor = ObjectProcessor;
    type EnumerationProcessor = EnumerationProcessor;
    type StructureProcessor = StructureProcessor;
    type ImplementationProcessor = ImplementationProcessor;
    type FunctionProcessor = FunctionProcessor;
    type ParameterProcessor = ParameterProcessor;
}

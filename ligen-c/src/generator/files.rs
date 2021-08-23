use ligen::generator::{ImplementationVisitor, FileProcessorVisitor, FileSet, FunctionVisitor, ParameterVisitor, FileGeneratorVisitors, StructureVisitor, ObjectVisitor, ModuleVisitor, ProjectVisitor};
use ligen::ir;
use std::path::PathBuf;
use crate::ast::{Types, Type};
use crate::generator::CGenerator;

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

/// Implementation processor.
#[derive(Default, Clone, Copy, Debug)]
pub struct ImplementationProcessor;

/// Function processor.
#[derive(Default, Clone, Copy, Debug)]
pub struct FunctionProcessor;

/// Parameter processor.
#[derive(Default, Clone, Copy, Debug)]
pub struct ParameterProcessor;

fn path(visitor: &ObjectVisitor) -> PathBuf {
    let mut path = PathBuf::from("include");
    for segment in &visitor.current.path.segments {
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

    fn process(&self, file_set: &mut FileSet, visitor: &Self::Visitor) {
        let file = file_set.entry(&path(&visitor));
        // includes
        file.writeln("#pragma once");
        file.writeln("");
        file.writeln("#include <stdint.h>");
        file.writeln("");
        file.writeln("#ifdef __cplusplus");
        file.writeln("extern \"C\" {");
        file.writeln("#endif\n");
    }

    fn post_process(&self, file_set: &mut FileSet, visitor: &Self::Visitor) {
        let file = file_set.entry(&path(&visitor));

        // drop function
        let object_name = &visitor.current.path.last().name;
        let c_type      = Type::from(ir::Type::Compound(visitor.current.path.clone()));
        file.writeln(format!("void {name}_drop({type_} self);", name = object_name, type_ = c_type));

        // epilogue
        file.writeln("");
        file.writeln("#ifdef __cplusplus");
        file.writeln("}");
        file.writeln("#endif");
    }
}

impl FileProcessorVisitor for StructureProcessor {
    type Visitor = StructureVisitor;

    fn process(&self, file_set: &mut FileSet, visitor: &Self::Visitor) {
        let file = file_set.entry(&path(&visitor.parent));
        file.writeln(format!("typedef struct Struct_{} {{", visitor.current.identifier));
        file.writeln("\tvoid* self;");
        file.writeln(format!("}} C{};", visitor.current.identifier));
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
    pub fn generate_function_name(&self, visitor: &FunctionVisitor) -> String {
        // FIXME: This naming convention happens in the extern generator and here. How can we generalize this code?
        format!("{}_{}", &visitor.parent.current.self_.path().last().name, &visitor.current.identifier.name)
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
                        "String" => "CRString".to_string(),
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

    fn process(&self, file_set: &mut FileSet, visitor: &Self::Visitor) {
        if let ir::Visibility::Public = visitor.current.visibility {
            let file = file_set.entry(&path(&visitor.parent.parent));
            file.write(self.generate_function_output(&visitor.current.output));
            file.write(self.generate_function_name(&visitor));
            file.write("(");
        }
    }

    fn post_process(&self, file_set: &mut FileSet, visitor: &Self::Visitor) {
        if let ir::Visibility::Public = visitor.current.visibility {
            let file = file_set.entry(&path(&visitor.parent.parent));
            file.writeln(");");
        }
    }
}

impl FileProcessorVisitor for ParameterProcessor {
    type Visitor = ParameterVisitor;

    fn process(&self, file_set: &mut FileSet, visitor: &Self::Visitor) {
        let file = file_set.entry(&path(&visitor.parent.parent.parent));

        let mut type_ = Type::from(visitor.current.type_.clone());
        if let (Some(_pointer), Types::Compound(_type)) = (&type_.pointer, &type_.type_) {
            type_.pointer = None;
        }
        let ident = &visitor.current.identifier.name;
        file.write(format!("{} {}", type_, ident))
    }

    fn post_process(&self, file_set: &mut FileSet, visitor: &Self::Visitor) {
        let file = file_set.entry(&path(&visitor.parent.parent.parent));
        file.write(", ");
    }
}

impl FileGeneratorVisitors for CGenerator {
    type ProjectProcessor = ProjectProcessor;
    type ModuleProcessor = ModuleProcessor;
    type ObjectProcessor = ObjectProcessor;
    type StructureProcessor = StructureProcessor;
    type ImplementationProcessor = ImplementationProcessor;
    type FunctionProcessor = FunctionProcessor;
    type ParameterProcessor = ParameterProcessor;
}

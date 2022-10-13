use ligen::traits::generator::{ImplementationVisitor, FileSet, FunctionVisitor, ParameterVisitor, FileGeneratorVisitors, StructureVisitor, ObjectVisitor, ModuleVisitor, ProjectVisitor, EnumerationVisitor, FunctionParent};
use ligen::ir;
use std::path::PathBuf;
use crate::ast::{Types, Type};
use crate::generator::CGenerator;
use ligen::ir::{Mutability, Path};
use ligen::traits::generator::file_processor_visitor::FileProcessorVisitor;

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

    fn process(&self, _file_set: &mut FileSet, _module: &Self::Visitor) {
        /*
        let file = file_set.entry(&path(&module.path()));
        file.writeln("#pragma once");
        for import in &module.current.imports {
            // TODO: Differ between absolute and relative imports. (crate::... == absolute, external_crate::... == absolute, super, self, relative)
            // This logic should belong somewhere else in `ligen`.
            let mut import = import.clone();
            import.path.segments.pop();
            let snake_case = SnakeCase::from(module.parent_project().name.clone());
            let path = if import.path.segments[0] == "crate".into() { // FIXME: "crate" is no longer used.
                import.path.segments[0].replace_identifier(&"crate".into(), &snake_case.to_string().into());
                import.path
            } else {
                let mut path = module.path();
                path.segments.append(&mut import.path.segments);
                path
            };
            let path: Vec<_> = path.segments.iter().map(|x| x.to_string()).collect();
            let path = path.join("/");
            file.writeln(format!("#include <{}.h>", path));
        }
        */
    }

    fn post_process(&self, _file_set: &mut FileSet, _visitor: &Self::Visitor) {}
}

impl FileProcessorVisitor for ObjectProcessor {
    type Visitor = ObjectVisitor;

    fn process(&self, _file_set: &mut FileSet, _object: &Self::Visitor) {
        // let file = file_set.entry(&path(&object.parent.path()));
        // includes
        // file.writeln("#pragma once");
        // file.writeln("");
        // file.writeln("#ifdef __cplusplus");
        // file.writeln("extern \"C\" {");
        // file.writeln("#endif\n");
    }

    fn post_process(&self, file_set: &mut FileSet, object: &Self::Visitor) {
        let file = file_set.entry(&path(&object.path()));

        // drop function
        let object_name = &object.path.last().name;
        let type_ = ir::Type::Compound(object.path.clone(), Default::default()).into();
        let reference = ir::Reference { mutability: Mutability::Mutable, kind: ir::ReferenceKind::Pointer, type_ };
        let c_type = Type::from(ir::Type::Reference(reference));
        file.writeln(format!("void {name}_drop({type_} self);", name = object_name, type_ = c_type));

        // epilogue
        // file.writeln("");
        // file.writeln("#ifdef __cplusplus");
        // file.writeln("}");
        // file.writeln("#endif");
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
        file.writeln("typedef struct {");
        file.writeln("\tvoid* opaque;");
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
            if path(&function.path()).display().to_string().contains("Instant.h") {
                panic!("{}", file.content);
            }
        }
    }
}

impl FileProcessorVisitor for ParameterProcessor {
    type Visitor = ParameterVisitor;

    fn process(&self, file_set: &mut FileSet, parameter: &Self::Visitor) {
        if let ir::Visibility::Public = parameter.parent.visibility {
            let file = file_set.entry(&path(&parameter.parent.path()));

            let type_ = Type::from(parameter.type_.clone());
            let ident = &parameter.identifier.name;
            file.write(format!("{} {}", type_, ident));
        }
    }

    fn post_process(&self, file_set: &mut FileSet, parameter: &Self::Visitor) {
        if let ir::Visibility::Public = parameter.parent.visibility {
            let file = file_set.entry(&path(&parameter.parent.path()));
            file.write(", ");
        }
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

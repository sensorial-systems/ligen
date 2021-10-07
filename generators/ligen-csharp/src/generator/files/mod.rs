use ligen::generator::{ImplementationVisitor, FileProcessorVisitor, FileSet, FunctionVisitor, ParameterVisitor, FileGeneratorVisitors, StructureVisitor, ObjectVisitor, ModuleVisitor, ProjectVisitor, EnumerationVisitor, FunctionParent};
use ligen::ir;
use crate::ast::Type;
use std::path::PathBuf;
use crate::generator::CSharpGenerator;
use ligen::conventions::naming::{PascalCase, SnakeCase};
use std::convert::TryFrom;
use ligen::ir::TypeDefinition;

mod project_processor;
mod module_processor;
mod object_processor;
mod structure_processor;
mod enumeration_processor;
mod implementation_processor;
mod function_processor;
mod parameter_processor;

pub use project_processor::*;
pub use module_processor::*;
pub use object_processor::*;
pub use structure_processor::*;
pub use enumeration_processor::*;
pub use implementation_processor::*;
pub use function_processor::*;
pub use parameter_processor::*;

fn path(module: &ModuleVisitor) -> PathBuf {
    let mut path = PathBuf::from("");
    let segments = module.path().segments;
    for segment in segments.iter().take(segments.len() - 1) {
        path = path.join(segment.to_string());
    }
    let name = segments.last().unwrap().clone().name;
    let snake_case = SnakeCase::try_from(name.as_str()).expect("Couldn't convert module name from snake_case to PascalCase.");
    let pascal_case = PascalCase::from(snake_case);
    path = path.join(format!("{}", pascal_case));
    path.with_extension("cs")
}

impl FileGeneratorVisitors for CSharpGenerator {
    type ProjectProcessor = ProjectProcessor;
    type ModuleProcessor = ModuleProcessor;
    type ObjectProcessor = ObjectProcessor;
    type EnumerationProcessor = EnumerationProcessor;
    type StructureProcessor = StructureProcessor;
    type ImplementationProcessor = ImplementationProcessor;
    type FunctionProcessor = FunctionProcessor;
    type ParameterProcessor = ParameterProcessor;
}

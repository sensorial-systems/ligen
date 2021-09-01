//! File generator with visitors.

use crate::generator::{FileSet, ProjectVisitor, ImplementationVisitor, FunctionVisitor, ParameterVisitor, FileProcessorVisitor, ObjectVisitor, StructureVisitor, ModuleVisitor, FileGenerator, EnumerationVisitor};
use crate::ir::{ImplementationItem, TypeDefinition};

/// File generator with visitors.
pub trait FileGeneratorVisitors {
    /// Project processor.
    type ProjectProcessor: FileProcessorVisitor<Visitor = ProjectVisitor>;

    /// Module processor.
    type ModuleProcessor: FileProcessorVisitor<Visitor = ModuleVisitor>;

    /// Object processor.
    type ObjectProcessor: FileProcessorVisitor<Visitor = ObjectVisitor>;

    /// Enumeration processor.
    type EnumerationProcessor: FileProcessorVisitor<Visitor = EnumerationVisitor>;

    /// Structure processor.
    type StructureProcessor: FileProcessorVisitor<Visitor = StructureVisitor>;

    /// Implementation processor.
    type ImplementationProcessor: FileProcessorVisitor<Visitor = ImplementationVisitor>;

    /// Function processor.
    type FunctionProcessor: FileProcessorVisitor<Visitor = FunctionVisitor>;

    /// Parameter processor.
    type ParameterProcessor: FileProcessorVisitor<Visitor = ParameterVisitor>;

    /// Process project.
    fn process_project(&self, file_set: &mut FileSet, visitor: &ProjectVisitor) {
        let project_processor = Self::ProjectProcessor::default();
        project_processor.process(file_set, &visitor);
        self.process_module(file_set, &visitor.child(visitor.root_module.clone()));
        project_processor.post_process(file_set, &visitor);
    }

    /// Process module.
    fn process_module<V: Into<ModuleVisitor>>(&self, file_set: &mut FileSet, visitor: V) {
        let visitor = &visitor.into();
        let module_processor = Self::ModuleProcessor::default();
        module_processor.process(file_set, visitor);
        for module in &visitor.modules {
            self.process_module(file_set, &visitor.child(module.clone()));
        }
        for object in &visitor.objects {
            self.process_object(file_set, &visitor.child(object.clone()));
        }
        module_processor.post_process(file_set, visitor);
    }

    /// Process object.
    fn process_object(&self, file_set: &mut FileSet, visitor: &ObjectVisitor) {
        let object_processor = Self::ObjectProcessor::default();
        object_processor.process(file_set, visitor);
        match &visitor.definition {
            TypeDefinition::Structure(structure) => self.process_structure(file_set, &visitor.child(structure.clone())),
            TypeDefinition::Enumeration(enumeration) => self.process_enumeration(file_set, &visitor.child(enumeration.clone()))
        }
        for implementation in &visitor.implementations {
            self.process_implementation(file_set, &visitor.child(implementation.clone()));
        }
        object_processor.post_process(file_set, visitor);
    }

    /// Process enumeration.
    fn process_enumeration(&self, file_set: &mut FileSet, visitor: &EnumerationVisitor) {
        let enumeration_processor = Self::EnumerationProcessor::default();
        enumeration_processor.process(file_set, visitor);
        enumeration_processor.post_process(file_set, visitor);
    }

    /// Process structure.
    fn process_structure(&self, file_set: &mut FileSet, visitor: &StructureVisitor) {
        let structure_processor = Self::StructureProcessor::default();
        structure_processor.process(file_set, visitor);
        structure_processor.post_process(file_set, visitor);
    }

    /// Process implementation.
    fn process_implementation(&self, file_set: &mut FileSet, visitor: &ImplementationVisitor) {
        let implementation_processor = Self::ImplementationProcessor::default();
        implementation_processor.process(file_set, visitor);
        for item in &visitor.items {
            match item {
                ImplementationItem::Constant(_) => (),
                ImplementationItem::Method(function) => self.process_function(file_set, &visitor.child(function.clone()))
            }
        }
        implementation_processor.post_process(file_set, visitor);
    }

    /// Process function.
    fn process_function(&self, file_set: &mut FileSet, visitor: &FunctionVisitor) {
        let function_processor = Self::FunctionProcessor::default();
        function_processor.process(file_set, visitor);
        for (index, parameter) in visitor.inputs.iter().enumerate() {
            let is_last = index == visitor.inputs.len() - 1;
            self.process_parameter(file_set, &visitor.child(parameter.clone()), is_last)
        }
        function_processor.post_process(file_set, visitor);
    }

    /// Process parameter.
    fn process_parameter(&self, file_set: &mut FileSet, visitor: &ParameterVisitor, is_last: bool) {
        let parameter_processor = Self::ParameterProcessor::default();
        parameter_processor.process(file_set, &visitor);
        if !is_last {
            parameter_processor.post_process(file_set, &visitor);
        }
    }
}

impl<T: FileGeneratorVisitors> FileGenerator for T {
    fn generate_files(&self, file_set: &mut FileSet, visitor: &ProjectVisitor) {
        self.process_project(file_set, visitor);
    }
}

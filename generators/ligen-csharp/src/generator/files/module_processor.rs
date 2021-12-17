use super::*;

/// Module processor.
#[derive(Default, Clone, Copy, Debug)]
pub struct ModuleProcessor;

impl FileProcessorVisitor for ModuleProcessor {
    type Visitor = ModuleVisitor;

    fn process(&self, file_set: &mut FileSet, visitor: &Self::Visitor) {
        let file = file_set.entry(&path(&visitor));
        let name = visitor.parent_project().name();
        let name = PascalCase::from(name.clone());
        file.writeln(format!("namespace {}", name));
        file.writeln("{");
        file.writeln("\tusing System;");
        file.writeln("\tusing System.Runtime.InteropServices;");
        file.writeln("\tusing System.Collections.Generic;");

    }

    fn post_process(&self, file_set: &mut FileSet, visitor: &Self::Visitor) {
        let file = file_set.entry(&path(&visitor));
        file.writeln("}");
    }
}

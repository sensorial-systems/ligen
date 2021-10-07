use super::*;

/// Enumeration processor.
#[derive(Default, Clone, Copy, Debug)]
pub struct EnumerationProcessor;

impl FileProcessorVisitor for EnumerationProcessor {
    type Visitor = EnumerationVisitor;

    fn process(&self, file_set: &mut FileSet, enumeration: &Self::Visitor) {
        let file = file_set.entry(&path(enumeration.parent_module()));
        for variant in &enumeration.variants {
            file.writeln(format!("\t\t{},", variant.identifier));
        }
    }

    fn post_process(&self, _file_set: &mut FileSet, _enumeration: &Self::Visitor) {}
}

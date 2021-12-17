use super::*;
use handlebars::Handlebars;
use serde_json::json;

/// Enumeration processor.
#[derive(Default, Clone, Copy, Debug)]
pub struct EnumerationProcessor;

impl FileProcessorVisitor for EnumerationProcessor {
    type Visitor = EnumerationVisitor;

    fn process(&self, file_set: &mut FileSet, enumeration: &Self::Visitor) {
        let renderer = Handlebars::new();
        let file = file_set.entry(&path(enumeration.parent_module()));
        let template = include_str!("enumeration.template.cs");
        for variant in &enumeration.variants {
            let values = json!({"identifier": variant.identifier.to_string()});
            file.writeln(renderer.render_template(template, &values).unwrap());
        }
    }

    fn post_process(&self, _file_set: &mut FileSet, _enumeration: &Self::Visitor) {}
}

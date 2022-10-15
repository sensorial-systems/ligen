use ligen_ir::*;

use ligen_traits::generator::{ProjectVisitor, Generator, FileSet, FileGenerator};
use std::path::PathBuf;
use std::str::FromStr;

use handlebars::{Context, Handlebars as Template, Handlebars, Helper, HelperResult, Output, RenderContext};
use ligen_ir::visitor::ModuleVisitor;
use ligen_traits::prelude::{Error, Result as LigenResult};

#[derive(Debug, Default)]
pub struct CGenerator;

impl CGenerator {
    pub fn get_template(&self) -> LigenResult<Template> {
        let mut template = Template::new();
        template.register_template_string("identifier", include_str!("templates/identifier.hbs")).expect("Failed to load identifier template.");
        template.register_template_string("arguments", include_str!("templates/arguments.hbs")).expect("Failed to load arguments template.");
        template.register_template_string("implementation", include_str!("templates/implementation.hbs")).expect("Failed to load implementation template.");
        template.register_template_string("method", include_str!("templates/method.hbs")).expect("Failed to load method template.");
        template.register_template_string("function", include_str!("templates/function.hbs")).expect("Failed to load method template.");
        template.register_template_string("module", include_str!("templates/module.hbs")).expect("Failed to load module template.");
        template.register_template_string("object", include_str!("templates/object.hbs")).expect("Failed to load object template.");
        template.register_template_string("parameters", include_str!("templates/parameters.hbs")).expect("Failed to load parameters template.");
        template.register_template_string("project", include_str!("templates/project.hbs")).expect("Failed to load project template.");
        Ok(template)
    }

    pub fn get_functions(&self, template: &mut Template, visitor: &ProjectVisitor) {
        let _root_module = visitor.current.root_module.clone();
        template.register_helper("marshal_type", Box::new(move |h: &Helper<'_, '_>, _: &Handlebars<'_>, _context: &Context, _rc: &mut RenderContext<'_, '_>, out: &mut dyn Output| -> HelperResult {
            let param = h
                .param(0)
                .map(|value| value.value().clone())
                .filter(|value| !value.is_null());
            let content = if let Some(_param) = param {
                // let type_ = serde_json::from_value::<Type>(param).unwrap();
                // let identifier = type_.path().last();
                "int".to_string()
            } else {
                "void".to_string()
            };
            out.write(&content)?;
            Ok(())
        }));
        template.register_helper("join_path", Box::new(move |h: &Helper<'_, '_>, _: &Handlebars<'_>, _context: &Context, _rc: &mut RenderContext<'_, '_>, out: &mut dyn Output| -> HelperResult {
            let separator = serde_json::from_value::<String>(h.param(0).unwrap().value().clone()).unwrap();
            let path = serde_json::from_value::<Path>(h.param(1).unwrap().value().clone()).unwrap();
            // TODO: Move the join logic to Path? Something like fn to_string(&self, separator: &str)?
            let content = path
                .segments
                .into_iter()
                .map(|identifier| identifier.name)
                .collect::<Vec<_>>()
                .join(&separator);
            out.write(&content)?;
            Ok(())
        }));
    }

    pub fn generate_module(&self, template: &Template, file_set: &mut FileSet, visitor: &ModuleVisitor) -> LigenResult<()> {
        let value = serde_json::to_value(&visitor.current)?;
        let content = template.render("module", &value).map_err(|e| Error::Message(format!("{}", e)))?;
        let mut path = PathBuf::from_str("include").unwrap();
        for segment in visitor.current.path.clone().segments {
            path = path.join(segment.name);
        }
        path = path.with_extension("h");
        file_set.entry(&path).writeln(content);
        for module in &visitor.current.modules {
            self.generate_module(template, file_set, &ModuleVisitor::from(&visitor.child(module.clone())))?;
        }
        Ok(())
    }
}

impl Generator for CGenerator {
    fn base_path(&self) -> PathBuf {
        PathBuf::from("c".to_string())
    }
}

impl FileGenerator for CGenerator {
    fn generate_files(&self, file_set: &mut FileSet, visitor: &ProjectVisitor) -> LigenResult<()> {
        let mut template = self.get_template()?;
        self.get_functions(&mut template, visitor);
        self.generate_module(&template, file_set, &ModuleVisitor::from(&visitor.child(visitor.root_module.clone())))?;
        Ok(())
    }
}

use ligen_traits::generator::{ProjectVisitor, Generator, FileSet, FileGenerator};
use std::path::PathBuf;
use std::str::FromStr;
use handlebars::handlebars_helper;
use ligen_ir::Type;
use serde_json::Value;
// use ligen_ir::Project;
// use ligen_traits::generator::{FunctionVisitor, File, ImplementationVisitor, ModuleVisitor, ObjectVisitor, FunctionParent};
// use ligen_ir::{Identifier, ImplementationItem, Visibility};

// /// A generic FFI generator which can be used for most languages.
// pub trait GenericFFIGenerator {
//     /// Generate the function parameters.
//     fn generate_parameters(_marshaller: &Marshaller, file: &mut File, visitor: &FunctionVisitor) {
//         for parameter in &visitor.current.inputs {
//             let root_module = &visitor.parent_module().parent_project().root_module;
//             let identifier = parameter.type_.path().last();
//             let is_opaque = root_module
//                 .get_literal_from_path(format!("ligen::ffi::{}::opaque", identifier))
//                 .map(|literal| literal.to_string() == "true")
//                 .unwrap_or_default();
//             let type_ = if is_opaque { parameter.type_.drop_reference() } else { parameter.type_.clone() };
//             let type_ = root_module
//                 .get_literal_from_path(format!("ligen::ffi::{}::name", identifier))
//                 .map(|literal| {
//                         type_
//                             .to_string()
//                             .replace(&identifier.name, &literal.to_string())
//                 })
//                 .unwrap_or(type_.to_string());
//             let identifier = &parameter.identifier.name.replace("self", "self_");
//             if is_opaque {
//                 file.write(format!("{identifier}: *mut {type_}, ", identifier = identifier, type_ = type_))
//             } else {
//                 file.write(format!("{identifier}: {type_}, ", identifier = identifier, type_ = type_))
//             }
//         }
//     }
//
//     /// Generate the function call arguments and its conversions.
//     fn generate_arguments(file: &mut File, function: &FunctionVisitor) {
//         for input in &function.current.inputs {
//             let identifier = &input.identifier.name.replace("self", "self_");
//             let type_identifier = input.type_.path().last();
//             let is_opaque = function
//                 .parent_module()
//                 .parent_project()
//                 .root_module
//                 .get_literal_from_path(format!("ligen::ffi::{}::opaque", type_identifier))
//                 .map(|literal| literal.to_string() == "true")
//                 .unwrap_or_default();
//             if is_opaque {
//                 if let ligen_ir::Type::Reference(_reference) = &input.type_ {
//                     file.write(format!("{identifier}.as_mut().unwrap(), ", identifier = identifier));
//                 } else {
//                     file.write(format!("{identifier}.read().marshal_into(), ", identifier = identifier));
//                 }
//             } else {
//                 file.write(format!("{identifier}.marshal_into(), ", identifier = identifier));
//             }
//         }
//     }
//
//     /// Generate the function output.
//     fn generate_output(_marshaller: &Marshaller, file: &mut File, visitor: &FunctionVisitor) {
//         match &visitor.current.output {
//             Some(type_) => {
//                 let identifier = type_.path().last();
//                 let root_module = &visitor.parent_module().parent_project().root_module;
//                 let is_opaque = root_module
//                     .get_literal_from_path(format!("ligen::ffi::{}::opaque", identifier))
//                     .map(|literal| literal.to_string() == "true")
//                     .unwrap_or_default();
//                 let type_ = root_module
//                     .get_literal_from_path(format!("ligen::ffi::{}::name", identifier)).map(|literal| type_.to_string().replace(&identifier.name, &literal.to_string())).unwrap_or(type_.to_string());
//                 if is_opaque {
//                     file.write(&format!(" -> *mut {}", type_))
//                 } else {
//                     file.write(&format!(" -> {}", type_))
//                 }
//             },
//             _ => ()
//         }
//     }
//
//     /// Generate the function
//     fn generate_function_signature(marshaller: &Marshaller, file: &mut File, visitor: &FunctionVisitor) {
//         let function_name = match &visitor.parent {
//             FunctionParent::Implementation(implementation) => {
//                 format!("{}_{}", implementation.self_.path().last().name, visitor.identifier.name)
//             },
//             FunctionParent::Module(_) => {
//                 visitor.identifier.name.clone()
//             }
//         };
//         let function_identifier = Identifier::new(&function_name);
//         file.writeln("#[allow(unused_unsafe)]");
//         file.writeln("#[no_mangle]");
//         file.write(format!("pub extern \"cdecl\" fn {function_identifier}(", function_identifier = function_identifier));
//         Self::generate_parameters(marshaller, file, visitor);
//         file.write(")");
//         Self::generate_output(marshaller, file, &visitor);
//     }
//
//     /// Generate the function
//     fn generate_function_block(_marshaller: &Marshaller, file: &mut File, visitor: &FunctionVisitor) {
//         let root_module = &visitor.parent_module().parent_project().root_module;
//         let method = &visitor.current;
//         let method_identifier = &method.identifier;
//         let function_path = format!("{}::{}", visitor.path(), method_identifier.name);
//         let is_opaque = if let Some(type_) = &visitor.current.output {
//             let identifier = type_.path().last();
//             root_module
//                 .get_literal_from_path(format!("ligen::ffi::{}::opaque", identifier))
//                 .map(|literal| literal.to_string() == "true")
//                 .unwrap_or_default()
//         } else {
//             false
//         };
//         file.writeln(" {");
//         file.writeln("\tlet result = unsafe {");
//         file.write(format!("\t\t{}(", function_path));
//         Self::generate_arguments(file, visitor);
//         file.writeln(")");
//         file.writeln("\t};");
//
//         if is_opaque {
//             file.writeln("\tBox::into_raw(Box::new(result.marshal_into()))");
//         } else {
//             file.writeln("\tresult.marshal_into()");
//         }
//         file.writeln("}");
//     }
//
//     /// Generate an extern function for an implementation method.
//     fn generate_function<V: Into<FunctionVisitor>>(marshaller: &Marshaller, file: &mut File, visitor: V) {
//         let visitor = &visitor.into();
//         if let Visibility::Public = visitor.current.visibility {
//             Self::generate_function_signature(marshaller, file, visitor);
//             Self::generate_function_block(marshaller, file, visitor);
//         }
//     }
//
//     /// Generate drop extern.
//     fn generate_drop(_file: &mut File, _visitor: &ObjectVisitor) {
//         // let self_path = &visitor.current.path;
//         // let object_name = self_path.last();
//         // let drop_name = Identifier::new(format!("{}_drop", object_name.name).as_str());
//         // file.writeln("#[no_mangle]");
//         // // FIXME: Hardcoded calling convention.
//         // file.writeln(format!("pub unsafe extern \"cdecl\" fn {}(object: *mut {}) {{", drop_name, object_name));
//         // file.writeln("\tBox::from_raw(object);");
//         // file.writeln("}");
//     }
//
//     /// Generate project externs.
//     fn generate(marshaller: &Marshaller, file: &mut File, visitor: &ProjectVisitor) {
//         file.writeln("#![allow(unused_imports)]");
//         Self::generate_module(marshaller, file, &visitor.child(visitor.current.root_module.clone()));
//     }
//
//     /// Generate module externs.
//     fn generate_module<V: Into<ModuleVisitor>>(marshaller: &Marshaller, file: &mut File, visitor: V) {
//         let visitor = &visitor.into();
//         // FIXME: How to implement Join<Separator> so we can reduce verbosity?
//         file.writeln("use ligen::marshalling::*;");
//         file.writeln(format!("use {}::*;", visitor.path().segments.iter().map(|x| x.name.clone()).collect::<Vec<_>>().join("::")));
//         file.writeln("");
//         for module in &visitor.current.modules {
//             Self::generate_module(marshaller, file, &visitor.child(module.clone()));
//         }
//         for object in &visitor.current.objects {
//             Self::generate_object(marshaller, file, &visitor.child(object.clone()));
//         }
//         for function in &visitor.current.functions {
//             Self::generate_function(marshaller, file, &visitor.child(function.clone()));
//         }
//     }
//
//     /// Generate object externs.
//     fn generate_object(marshaller: &Marshaller, file: &mut File, visitor: &ObjectVisitor) {
//         for implementation in &visitor.current.implementations {
//             Self::generate_implementation(marshaller, file, &visitor.child(implementation.clone()));
//         }
//         Self::generate_drop(file, visitor);
//     }
//
//     /// Generate externs for Constants and Methods.
//     fn generate_implementation(marshaller: &Marshaller, file: &mut File, visitor: &ImplementationVisitor) {
//         for item in &visitor.current.items {
//             match item {
//                 ImplementationItem::Constant(_) => (),
//                 ImplementationItem::Method(method) => Self::generate_function(marshaller, file, &visitor.child(method.clone())),
//             }
//         }
//     }
// }

use handlebars::Handlebars as Template;
use handlebars::*;
use ligen_traits::prelude::{Error, Result as LigenResult};

#[derive(Debug, Default)]
pub struct RustGenerator;

impl RustGenerator {
    pub fn get_template(&self) -> LigenResult<Template> {
        let mut template = Template::new();
        template.register_template_string("identifier", include_str!("templates/identifier.hbs")).expect("Failed to load identifier template.");
        template.register_template_string("arguments", include_str!("templates/arguments.hbs")).expect("Failed to load arguments template.");
        template.register_template_string("implementation", include_str!("templates/implementation.hbs")).expect("Failed to load implementation template.");
        template.register_template_string("import", include_str!("templates/import.hbs")).expect("Failed to load import template.");
        template.register_template_string("method", include_str!("templates/method.hbs")).expect("Failed to load method template.");
        template.register_template_string("module", include_str!("templates/module.hbs")).expect("Failed to load module template.");
        template.register_template_string("object", include_str!("templates/object.hbs")).expect("Failed to load object template.");
        template.register_template_string("parameters", include_str!("templates/parameters.hbs")).expect("Failed to load parameters template.");
        template.register_template_string("project", include_str!("templates/project.hbs")).expect("Failed to load project template.");
        Ok(template)
    }

    pub fn get_functions(&self, template: &mut Template, visitor: &ProjectVisitor) {
        handlebars_helper!(display_type: |value: Value| serde_json::from_value::<Type>(value).unwrap().to_string());
        let root_module = visitor.current.root_module.clone();
        template.register_helper("marshal_type", Box::new(move |h: &Helper<'_, '_>, _: &Handlebars<'_>, _context: &Context, _rc: &mut RenderContext<'_, '_>, out: &mut dyn Output| -> HelperResult {
            let param = h
                .param(0)
                .map(|value| value.value().clone())
                .filter(|value| !value.is_null());
            let content = if let Some(param) = param {
                let type_ = serde_json::from_value::<Type>(param).unwrap();
                let identifier = type_.path().last();
                let is_opaque = root_module
                    .get_literal_from_path(format!("ligen::ffi::{}::opaque", identifier))
                    .map(|literal| literal.to_string() == "true")
                    .unwrap_or_default();
                let (type_, opacity) = if is_opaque {
                    (type_.drop_reference().to_string(), "*mut ")
                } else {
                    ("*const u8".to_string(), "")
                };
                format!("{}{}", opacity, type_)
            } else {
                format!("()")
            };
            out.write(&content)?;
            Ok(())
        }));
        template.register_helper("display_type", Box::new(display_type));
    }
}

impl Generator for RustGenerator {
    fn base_path(&self) -> PathBuf {
        PathBuf::from("rust".to_string())
    }
}

impl FileGenerator for RustGenerator {
    fn generate_files(&self, file_set: &mut FileSet, visitor: &ProjectVisitor) -> LigenResult<()> {
        let mut template = self.get_template()?;
        self.get_functions(&mut template, visitor);
        let value = serde_json::to_value(&visitor.current)?;
        let content = template.render("project", &value).map_err(|e| Error::Message(format!("{}", e)))?;
        let file = file_set.entry(&PathBuf::from_str("src").unwrap().join("lib.rs"));
        file.writeln(content);
        Ok(())
    }
}

#[cfg(test)]
mod tests;
pub use crate::prelude::*;

use egui::CollapsingHeader;
use crate::ligen_editor::ui::{EditableList, Attributes, Import, Visibility, Constant, Function, Object, Identifier};

pub struct Module {}

impl Module {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, module: &mut ligen_ir::Module) {
        CollapsingHeader::new(module.identifier.to_string())
            .id_source("module")
            .show(ui, |ui| {
                ui.horizontal_top(|ui| {
                    Visibility::new().show(ui, &mut module.visibility);
                    Identifier::new().show(ui, &mut module.identifier);
                });
                EditableList::new("Imports", "Add import").show(ui, &mut module.imports, |ui, import| {
                    Import::new().show(ui, import);
                });
                EditableList::new("Constants", "Add constant").show(ui, &mut module.constants, |ui, constant| {
                    Constant::new().show(ui, constant);
                });
                EditableList::new("Functions", "Add function").show(ui, &mut module.functions, |ui, function| {
                    Function::new().show(ui, function);
                });
                EditableList::new("Modules", "Add module").show(ui, &mut module.modules, |ui, module| {
                    Module::new().show(ui, module);
                });
                EditableList::new("Objects", "Add object").show(ui, &mut module.objects, |ui, object| {
                    Object::new().show(ui, object);
                });
                Attributes::new().show(ui, &mut module.attributes);
            });
    }
}
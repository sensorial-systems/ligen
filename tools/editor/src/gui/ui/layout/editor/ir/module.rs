pub use crate::prelude::*;

use egui::CollapsingHeader;
use crate::gui::ui::EditableList;
use crate::gui::ui::editor::ir::{Attributes, Import, Visibility, Object, Function, Identifier, Type, TypeDefinition, Interface};

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
                EditableList::new("Types", "Add type").show(ui, &mut module.types, |ui, type_| {
                    TypeDefinition::new().show(ui, type_);
                });
                EditableList::new("Imports", "Add import").show(ui, &mut module.imports, |ui, import| {
                    Import::new().show(ui, import);
                });
                EditableList::new("Objects", "Add object").show(ui, &mut module.objects, |ui, object| {
                    Object::new().show(ui, object);
                });
                EditableList::new("Functions", "Add function").show(ui, &mut module.functions, |ui, function| {
                    Function::new().show(ui, function);
                });
                EditableList::new("Modules", "Add module").show(ui, &mut module.modules, |ui, module| {
                    Module::new().show(ui, module);
                });
                EditableList::new("Interfaces", "Add interface").show(ui, &mut module.interfaces, |ui, interface| {
                    Interface::new().show(ui, interface);
                });
                Attributes::new().show(ui, &mut module.attributes);
            });
    }
}
use crate::gui::egui::CollapsingHeader;
use crate::gui::ui::EditableList;
use crate::gui::ui::editor::ir::Identifier;
use crate::gui::ui::editor::symbols::interface::Interface;
use crate::prelude::*;

pub struct Module;

impl Module {
    pub fn new() -> Self {
        Self
    }

    pub fn show(&mut self, ui: &mut egui::Ui, module: &mut ligen_symbols::module::Module) {
        CollapsingHeader::new(module.identifier.to_string())
            .id_source("module")
            .show(ui, |ui| {
                Identifier::new().show(ui, &mut module.identifier);
                EditableList::new("Constants", "Add constant").show(ui, &mut module.constants, |ui, constant| {
                    Identifier::new().show(ui, constant);
                });
                EditableList::new("Functions", "Add function").show(ui, &mut module.functions, |ui, function| {
                    Identifier::new().show(ui, function);
                });
                EditableList::new("Types", "Add type").show(ui, &mut module.types, |ui, type_| {
                    Identifier::new().show(ui, type_);
                });
                EditableList::new("Interfaces", "Add interface").show(ui, &mut module.interfaces, |ui, interface| {
                    Interface::new().show(ui, interface);
                });
                EditableList::new("Modules", "Add module").show(ui, &mut module.modules, |ui, module| {
                    Module::new().show(ui, module);
                });
            });
    }
}
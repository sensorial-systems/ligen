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

    pub fn show(&mut self, ui: &mut egui::Ui, module: &mut ligen_symbols::Module) {
        CollapsingHeader::new(format!("{} - Symbols: {}", module.identifier, module.count_symbols()))
            .id_source("module")
            .show(ui, |ui| {
                EditableList::new(format!("Constants - Symbols: {}", module.constants.len()), "Add constant").show(ui, &mut module.constants, |ui, constant| {
                    Identifier::new().show(ui, constant);
                });
                EditableList::new(format!("Functions - Symbols: {}", module.functions.len()), "Add function").show(ui, &mut module.functions, |ui, function| {
                    Identifier::new().show(ui, function);
                });
                EditableList::new(format!("Types - Symbols: {}", module.types.len()), "Add type").show(ui, &mut module.types, |ui, type_| {
                    Identifier::new().show(ui, type_);
                });
                EditableList::new(format!("Interfaces - Symbols: {}", module.count_symbols_in_interfaces()), "Add interface").show(ui, &mut module.interfaces, |ui, interface| {
                    Interface::new().show(ui, interface);
                });
                EditableList::new(format!("Modules - Symbols: {}", module.count_symbols_in_modules()), "Add module").show(ui, &mut module.modules, |ui, module| {
                    Module::new().show(ui, module);
                });
            });
    }
}
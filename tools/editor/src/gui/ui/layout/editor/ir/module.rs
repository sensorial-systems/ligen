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
        CollapsingHeader::new(format!("{} - Symbols: {}", module.identifier, module.count_symbols()))
            .id_source("module")
            .show(ui, |ui| {
                ui.horizontal_top(|ui| {
                    Visibility::new().show(ui, &mut module.visibility);
                    Identifier::new().show(ui, &mut module.identifier);
                });
                EditableList::new(format!("Types - Symbols: {}", module.types.len()), "Add type").show(ui, &mut module.types, |ui, type_| {
                    TypeDefinition::new().show(ui, type_);
                });
                EditableList::new(format!("Objects - Symbols: {}", module.objects.len()), "Add object").show(ui, &mut module.objects, |ui, object| {
                    Object::new().show(ui, object);
                });
                EditableList::new(format!("Functions - Symbols: {}", module.functions.len()), "Add function").show(ui, &mut module.functions, |ui, function| {
                    Function::new().show(ui, function);
                });
                EditableList::new(format!("Interfaces - Symbols: {}", module.count_symbols_in_interfaces()), "Add interface").show(ui, &mut module.interfaces, |ui, interface| {
                    Interface::new().show(ui, interface);
                });
                EditableList::new(format!("Modules - Symbols: {}", module.count_symbols_in_modules()), "Add module").show(ui, &mut module.modules, |ui, module| {
                    Module::new().show(ui, module);
                });
                EditableList::new("Imports", "Add import").show(ui, &mut module.imports, |ui, import| {
                    Import::new().show(ui, import);
                });
                Attributes::new().show(ui, &mut module.attributes);
            });
    }
}
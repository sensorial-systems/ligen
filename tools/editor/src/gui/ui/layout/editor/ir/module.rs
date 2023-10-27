use crate::gui::ui::editor::settings::Settings;
use crate::gui::ui::editor::widget::Widget;
pub use crate::prelude::*;

use egui::CollapsingHeader;
use crate::gui::ui::EditableList;
use crate::gui::ui::editor::ir::{Attributes, Import, Visibility, Object, Function, Identifier, Type, TypeDefinition, Interface};

pub struct Module {}

impl Module {
    pub fn new() -> Self {
        Self {}
    }
}

impl Module {
    fn display_count(&self, name: &str, settings: &Settings, count: usize) -> String {
        if settings.display.show_symbols_count {
            format!("{} - Symbols: {}", name, count)
        } else {
            name.to_string()
        }
    }
}

impl Widget for Module {
    type Input = ligen_ir::Module;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, module: &mut ligen_ir::Module) {
        let text = if settings.display.show_visibility {
            format!("{} {}", module.visibility, module.identifier)
        } else {
            module.identifier.to_string()
        };
        let text = self.display_count(&text, settings, module.count_symbols());
        CollapsingHeader::new(text)
            .id_source("module")
            .show(ui, |ui| {
                if settings.editor.editable_fields {
                    ui.horizontal_top(|ui| {
                        Visibility::new().show(settings, ui, &mut module.visibility);
                        Identifier::new().show(settings, ui, &mut module.identifier);
                    });
                }
                EditableList::new(self.display_count("Types", settings, module.types.len()), "Add type").show(settings, ui, &mut module.types, |ui, type_| {
                    TypeDefinition::new().show(settings, ui, type_);
                });
                EditableList::new(self.display_count("Objects", settings, module.objects.len()), "Add object").show(settings, ui, &mut module.objects, |ui, object| {
                    Object::new().show(settings, ui, object);
                });
                EditableList::new(self.display_count("Functions", settings, module.functions.len()), "Add function").show(settings, ui, &mut module.functions, |ui, function| {
                    Function::new().show(settings, ui, function);
                });
                EditableList::new(self.display_count("Interfaces", settings, module.count_symbols_in_interfaces()), "Add interface").show(settings, ui, &mut module.interfaces, |ui, interface| {
                    Interface::new().show(settings, ui, interface);
                });
                EditableList::new(self.display_count("Modules", settings, module.count_symbols_in_modules()), "Add module").show(settings, ui, &mut module.modules, |ui, module| {
                    Module::new().show(settings, ui, module);
                });
                EditableList::new("Imports", "Add import").show(settings, ui, &mut module.imports, |ui, import| {
                    Import::new().show(settings, ui, import);
                });
                Attributes::new().show(settings, ui, &mut module.attributes);
            });
    }
}
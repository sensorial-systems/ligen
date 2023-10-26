use crate::{prelude::*, gui::ui::{EditableList, editor::{widget::Widget, settings::Settings}}};

use super::{Attributes, Method, Function, Object, Path, Identifier, Visibility};

#[derive(Default)]
pub struct Interface;

impl Interface {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Widget for Interface {
    type Input = ligen_ir::Interface;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, interface: &mut ligen_ir::Interface) {
        ui.vertical(|ui| {
            ui.horizontal_top(|ui| {
                Visibility::new().show(settings, ui, &mut interface.visibility);
                Identifier::new().show(settings, ui, &mut interface.identifier);    
            });
            EditableList::new(format!("Objects - Symbols: {}", interface.objects.len()), "Add object").show(settings, ui, &mut interface.objects, |ui, object| {
                Object::new().show(settings, ui, object);
            });
            EditableList::new(format!("Functions - Symbols: {}", interface.functions.len()), "Add function").show(settings, ui, &mut interface.functions, |ui, function| {
                Function::new().show(settings, ui, function);
            });
            EditableList::new(format!("Methods - Symbols: {}", interface.methods.len()), "Add method").show(settings, ui, &mut interface.methods, |ui, function| {
                Method::new().show(settings, ui, function);
            });
            EditableList::new("Interfaces", "Add interface").show(settings, ui, &mut interface.interfaces, |ui, interface| {
                Path::new().show(settings, ui, interface);
            });
            Attributes::new().show(settings, ui, &mut interface.attributes);    
        });
    }
}
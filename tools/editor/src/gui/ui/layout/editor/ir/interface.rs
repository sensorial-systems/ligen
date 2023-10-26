use crate::{prelude::*, gui::ui::EditableList};

use super::{Attributes, Method, Function, Object, Path, Identifier, Visibility};

#[derive(Default)]
pub struct Interface {
    editable: bool
}

impl Interface {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn editable(&mut self, editable: bool) -> &mut Self {
        self.editable = editable;
        self
    }

    pub fn show(&mut self, ui: &mut egui::Ui, interface: &mut ligen_ir::Interface) {
        ui.vertical(|ui| {
            ui.horizontal_top(|ui| {
                Visibility::new().show(ui, &mut interface.visibility);
                Identifier::new().show(ui, &mut interface.identifier);    
            });
            EditableList::new(format!("Objects - Symbols: {}", interface.objects.len()), "Add object").show(ui, &mut interface.objects, |ui, object| {
                Object::new().show(ui, object);
            });
            EditableList::new(format!("Functions - Symbols: {}", interface.functions.len()), "Add function").show(ui, &mut interface.functions, |ui, function| {
                Function::new().show(ui, function);
            });
            EditableList::new(format!("Methods - Symbols: {}", interface.methods.len()), "Add method").show(ui, &mut interface.methods, |ui, function| {
                Method::new().show(ui, function);
            });
            EditableList::new("Interfaces", "Add interface").show(ui, &mut interface.interfaces, |ui, interface| {
                Path::new().show(ui, interface);
            });
            Attributes::new().show(ui, &mut interface.attributes);    
        });
    }
}
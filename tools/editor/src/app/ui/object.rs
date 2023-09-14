use crate::app::ui::{Constant, EditableList, Function, Method, TypeDefinition};

pub struct Object {

}

impl Object {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, object: &mut ligen_ir::Object) {
        ui.vertical(|ui| {
            TypeDefinition::new().show(ui, &mut object.definition);
            EditableList::new("Constants", "Add constant").show(ui, &mut object.constants, |ui, constant| {
                Constant::new().show(ui, constant);
            });
            EditableList::new("Functions", "Add function").show(ui, &mut object.functions, |ui, function| {
                Function::new().show(ui, function);
            });
            EditableList::new("Methods", "Add method").show(ui, &mut object.methods, |ui, method| {
                Method::new().show(ui, method);
            });
        });
    }
}
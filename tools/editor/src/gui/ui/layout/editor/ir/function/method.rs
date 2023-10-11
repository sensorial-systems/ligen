use crate::gui::ui::{EditableList, OptionalField};
use crate::gui::ui::editor::ir::{Attributes, Identifier, Parameter, Synchrony, Type, Visibility};
pub use crate::prelude::*;

pub struct Method {

}

impl Method {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, method: &mut ligen_ir::Method) {
        ui.horizontal_top(|ui| {
            Type::new().set_enabled(false).show(ui, &mut method.owner);
            Visibility::new().show(ui, &mut method.visibility);
            Synchrony::new().show(ui, &mut method.synchrony);
            Identifier::new().show(ui, &mut method.identifier);
            EditableList::new("Inputs", "Add input").show(ui, &mut method.inputs, |ui, parameter| {
                Parameter::new().show(ui, parameter);
            });
            OptionalField::new("Output").show(ui, &mut method.output, |ui, output| {
                Type::new().show(ui, output);
            });
            Attributes::new().show(ui, &mut method.attributes);
        });
    }
}
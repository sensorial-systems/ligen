mod synchrony;
mod parameter;
mod method;

pub use method::*;
pub use synchrony::*;
pub use parameter::*;

use crate::app::ui::{Attributes, OptionalField, Visibility, Type, EditableList, Identifier};

pub struct Function {

}

impl Function {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, function: &mut ligen_ir::Function) {
        ui.horizontal_top(|ui| {
            Visibility::new().show(ui, &mut function.visibility);
            Synchrony::new().show(ui, &mut function.synchrony);
            Identifier::new().show(ui, &mut function.identifier);
            EditableList::new("Inputs", "Add input").show(ui, &mut function.inputs, |ui, parameter| {
                Parameter::new().show(ui, parameter);
            });
            OptionalField::new("Output").show(ui, &mut function.output, |ui, output| {
                Type::new().show(ui, output);
            });
            Attributes::new().show(ui, &mut function.attributes);
        });
    }
}
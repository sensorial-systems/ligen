pub use crate::prelude::*;

use crate::gui::ui::{Attributes, Identifier, Type};

pub struct Parameter {

}

impl Parameter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, parameter: &mut ligen_ir::Parameter) {
        ui.horizontal_top(|ui| {
            Type::new().show(ui, &mut parameter.type_);
            Identifier::new().show(ui, &mut parameter.identifier);
            Attributes::new().show(ui, &mut parameter.attributes);
        });
    }
}
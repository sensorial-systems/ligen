pub use crate::prelude::*;

use crate::gui::ui::editor::ir::{Identifier, Literal, Type};

pub struct Object {}

impl Object {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, object: &mut ligen_ir::Object) {
        ui.horizontal_top(|ui| {
            Identifier::new().show(ui, &mut object.identifier);
            Type::new().show(ui, &mut object.type_);
            if !object.literal.is_compatible_with(&object.type_) {
                object.literal = ligen_ir::Literal::default_for_type(&object.type_);
            }
            ui.label("=");
            Literal::new().editable(false).show(ui, &mut object.literal);
        });
    }
}
pub use crate::prelude::*;

use crate::gui::ui::editor::ir::{Identifier, Literal, Type};

pub struct Constant {}

impl Constant {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, constant: &mut ligen_ir::Constant) {
        ui.horizontal_top(|ui| {
            Identifier::new().show(ui, &mut constant.identifier);
            Type::new().show(ui, &mut constant.type_);
            if !constant.literal.is_compatible_with(&constant.type_) {
                constant.literal = ligen_ir::Literal::default_for_type(&constant.type_);
            }
            ui.label("=");
            Literal::new().switchable(false).show(ui, &mut constant.literal);
        });
    }
}
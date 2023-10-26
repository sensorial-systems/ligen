pub use crate::prelude::*;

use crate::gui::ui::editor::{ir::{Identifier, Literal, Type}, settings::Settings, widget::Widget};

#[derive(Default)]
pub struct Object;

impl Object {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Widget for Object {
    type Input = ligen_ir::Object;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, object: &mut ligen_ir::Object) {
        ui.horizontal_top(|ui| {
            Identifier::new().show(settings, ui, &mut object.identifier);
            Type::new().show(settings, ui, &mut object.type_);
            if !object.literal.is_compatible_with(&object.type_) {
                object.literal = ligen_ir::Literal::default_for_type(&object.type_);
            }
            ui.label("=");
            Literal::new().show(settings, ui, &mut object.literal);
        });
    }
}
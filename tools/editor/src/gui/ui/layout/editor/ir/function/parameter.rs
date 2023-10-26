pub use crate::prelude::*;

use crate::gui::ui::editor::{ir::{Attributes, Identifier, Type}, widget::Widget, settings::Settings};

#[derive(Default)]
pub struct Parameter;

impl Parameter {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Widget for Parameter {
    type Input = ligen_ir::Parameter;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, parameter: &mut ligen_ir::Parameter) {
        ui.horizontal_top(|ui| {
            Type::new().show(settings, ui, &mut parameter.type_);
            Identifier::new().show(settings, ui, &mut parameter.identifier);
            Attributes::new().show(settings, ui, &mut parameter.attributes);
        });
    }
}
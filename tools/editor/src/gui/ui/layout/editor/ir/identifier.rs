pub use crate::prelude::*;

use crate::gui::ui::{StringField, editor::{widget::Widget, settings::Settings}};


#[derive(Default)]
pub struct Identifier {
    string_field: StringField
}

impl Identifier {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Widget for Identifier {
    type Input = ligen_ir::Identifier;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, identifier: &mut ligen_ir::Identifier) {
        self.string_field.show(settings, ui, identifier)
    }
}
use crate::gui::ui::editor::settings::Settings;
use crate::gui::ui::editor::widget::Widget;
pub use crate::prelude::*;

use crate::gui::ui::editor::ir::{Attributes, Identifier, Type};
use crate::gui::ui::OptionalField;

#[derive(Default)]
pub struct Field;

impl Field {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Widget for Field {
    type Input = ligen_idl::Field;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, field: &mut ligen_idl::Field) {
        OptionalField::new("Identifier").show(settings, ui, &mut field.identifier, |ui, identifier| {
            Identifier::new().show(settings, ui, identifier);
        });
        Type::new().show(settings, ui, &mut field.type_);
        Attributes::new().show(settings, ui, &mut field.attributes);
    }
}
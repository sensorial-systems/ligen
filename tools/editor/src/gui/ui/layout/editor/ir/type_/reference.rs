use crate::gui::ui::editor::{ir::Type, widget::Widget, settings::Settings};
pub use crate::prelude::*;

use crate::gui::ui::EnumField;

#[derive(Default)]
pub struct Reference;

impl Reference {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Widget for Reference {
    type Input = ligen_ir::Reference;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, type_: &mut ligen_ir::Reference) {
        ui.horizontal_top(|ui| {
            EnumField::new().show(settings, ui, &mut type_.mutability);
            Type::new().show(settings, ui, &mut type_.type_);
        });
    }
}
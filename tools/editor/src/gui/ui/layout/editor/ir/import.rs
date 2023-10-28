use crate::gui::ui::editor::settings::Settings;
use crate::gui::ui::editor::widget::{Widget, WidgetFor};
pub use crate::prelude::*;

use crate::gui::ui::editor::ir::{Attributes, Identifier, Path, Visibility};
use crate::gui::ui::OptionalField;

#[derive(Default)]
pub struct Import;

impl Import {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Widget for Import {
    type Input = ligen_ir::Import;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, import: &mut ligen_ir::Import) {
        ui.horizontal_top(|ui| {
            Visibility::new().show(settings, ui, &mut import.visibility);
            Path::new().show(settings, ui, &mut import.path);
            OptionalField::new("as").show(settings, ui, &mut import.renaming, |ui, renaming| {
                Identifier::new().show(settings, ui, renaming);
            });
            Attributes::new().show(settings, ui, &mut import.attributes);
        });
    }
}

impl WidgetFor for ligen_ir::Import {
    type Widget = Import;
}
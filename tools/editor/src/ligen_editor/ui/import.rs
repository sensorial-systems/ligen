pub use crate::prelude::*;

use crate::ligen_editor::ui::{Attributes, Identifier, OptionalField, Path, Visibility};

pub struct Import {}

impl Import {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, import: &mut ligen_ir::Import) {
            ui.horizontal_top(|ui| {
                Visibility::new().show(ui, &mut import.visibility);
                Path::new().show(ui, &mut import.path);
                OptionalField::new("as").show(ui, &mut import.renaming, |ui, renaming| {
                    Identifier::new().show(ui, renaming);
                });
                Attributes::new().show(ui, &mut import.attributes);
            });
    }
}
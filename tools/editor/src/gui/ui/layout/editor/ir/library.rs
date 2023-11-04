use crate::gui::ui::editor::settings::Settings;
use crate::gui::ui::editor::widget::Widget;
pub use crate::prelude::*;

use egui::CollapsingHeader;
use crate::gui::ui::editor::ir::{Module, Directory};
use crate::gui::ui::StringField;

#[derive(Default)]	
pub struct Library;

impl Library {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Widget for Library {
    type Input = ligen_ir::Library;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, library: &mut Self::Input) {
        egui::ScrollArea::both()
            .auto_shrink([false, true])
            .show(ui, |ui| {
            CollapsingHeader::new(library.identifier.to_string())
                .id_source("library")
                .show(ui, |ui| {
                    if settings.editor.editable_fields {
                        StringField::new()
                        .show(settings, ui, &mut library.identifier);
                    }
                    Module::new().show(settings, ui, &mut library.root_module);
                });
        });
    }
}
use crate::gui::ui::editor::settings::Settings;
use crate::gui::ui::editor::widget::Widget;
pub use crate::prelude::*;

use egui::CollapsingHeader;
use crate::gui::ui::editor::ir::{Module, Directory};
use crate::gui::ui::StringField;

#[derive(Default)]	
pub struct Project;

impl Project {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Widget for Project {
    type Input = ligen_ir::Project;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, project: &mut Self::Input) {
        egui::ScrollArea::both()
            .auto_shrink([false, true])
            .show(ui, |ui| {
            CollapsingHeader::new(project.name.to_string())
                .id_source("project")
                .show(ui, |ui| {
                    if settings.editor.editable_fields {
                        StringField::new()
                        .show(settings, ui, &mut project.name);
                    }
                    Module::new().show(settings, ui, &mut project.root_module);
                });
        });
    }
}
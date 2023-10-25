pub use crate::prelude::*;

use egui::CollapsingHeader;
use crate::gui::ui::editor::ir::{Module, Directory};
use crate::gui::ui::StringField;

#[derive(Default)]	
pub struct Project {
    editable: bool
}

impl Project {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn editable(&mut self, editable: bool) -> &mut Self {
        self.editable = editable;
        self
    }

    pub fn show(&mut self, ui: &mut egui::Ui, project: &mut ligen_ir::Project) {
        egui::ScrollArea::both()
            .auto_shrink([false, true])
            .show(ui, |ui| {
            CollapsingHeader::new(project.name.to_string())
                .id_source("project")
                .show(ui, |ui| {
                    if self.editable {
                        StringField::new()
                        .editable(true)
                        .show(ui, &mut project.name);
                    }
                    Module::new().show(ui, &mut project.root_module);
                });
        });
    }
}

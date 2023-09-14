use egui::CollapsingHeader;
use crate::app::ui::{StringEditableField, Module, Directory};

pub struct Project {

}

impl Project {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, project: &mut ligen_ir::Project) {
        CollapsingHeader::new(&project.name.to_string())
            .id_source("project.name")
            .show(ui, |ui| {
                StringEditableField::new().show(ui, &mut project.name);
                Directory::new().show(ui, &mut project.directory);
                Module::new().show(ui, &mut project.root_module);
            });
    }
}
use ligen_gui_runtime::egui::CollapsingHeader;

use crate::{prelude::*, gui::ui::{editor::widget::Widget, StringField, Labeled}};

#[derive(Default)]
pub struct Language {}

impl Language {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Widget for Language {
    type Input = ligen_ir::Language;
    fn show(&mut self, settings: &crate::gui::ui::editor::settings::Settings, ui: &mut egui::Ui, input: &mut Self::Input) {
        CollapsingHeader::new("Language")
            .default_open(false)
            .show(ui, |ui| {
                Labeled::new("Name").show(settings, ui, |ui| {
                    StringField::default().show(settings, ui, &mut input.name);
                });
                Labeled::new("Requirement").show(settings, ui, |ui| {
                    // TODO: Add a requirement field.
                    // StringField::default().show(settings, ui, &mut input.requirement);
                });        
            });
    }
}

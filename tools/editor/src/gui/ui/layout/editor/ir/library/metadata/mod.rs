pub mod language;
pub use language::*;

pub mod author;
pub use author::*;

pub mod dependency;
pub use dependency::*;

use ligen_gui_runtime::egui::CollapsingHeader;

use crate::{prelude::*, gui::ui::{editor::{widget::Widget, settings::Settings}, StringField, SubWidgets}};
use crate::gui::ui::Labeled;

#[derive(Default)]
pub struct Metadata {}

impl Metadata {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Widget for Metadata {
    type Input = ligen_ir::Metadata;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, input: &mut Self::Input) {
        CollapsingHeader::new("Metadata")
                .show(ui, |ui| {
                    Labeled::new("Version").show(settings, ui, |ui| {
                        StringField::default().show(settings, ui, &mut input.version);    
                    });
                    Labeled::new("Homepage").show(settings, ui, |ui| {
                        StringField::default().show(settings, ui, &mut input.homepage);
                    });
                    Labeled::new("Summary").show(settings, ui, |ui| {
                        StringField::default().show(settings, ui, &mut input.summary);
                    });
                    Language::new().show(settings, ui, &mut input.language);
                    SubWidgets::new("Author").show(settings, ui, &mut input.authors);
                    SubWidgets::new_irregular("Dependency", "Dependencies").show(settings, ui, &mut input.dependencies);
                    CollapsingHeader::new("Description").show(ui, |ui| {
                        StringField::default().show(settings, ui, &mut input.description);
                    });
            });
    }
}
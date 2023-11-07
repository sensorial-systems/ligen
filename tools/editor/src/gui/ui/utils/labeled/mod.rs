use crate::prelude::*;
use crate::gui::ui::editor::{widget::Widget, settings::Settings};


pub struct Labeled {
    label: String
}

impl Labeled {
    pub fn new(label: impl Into<String>) -> Self {
        let label = label.into();
        Self { label }
    }

    pub fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, mut show: impl FnMut(&mut egui::Ui)) {
        ui.horizontal(|ui| {
            ui.label(&self.label);
            show(ui);
        });

    }
}

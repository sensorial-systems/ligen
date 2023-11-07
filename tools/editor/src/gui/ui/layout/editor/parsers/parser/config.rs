use crate::{prelude::*, gui::ui::editor::ir::{Path, Literal}};
use ligen_parser::ParserConfigSet;

use crate::gui::ui::editor::{widget::Widget, settings::Settings};

#[derive(Default)]
pub struct ParserConfig {}

impl ParserConfig {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Widget for ParserConfig {
    type Input = ligen_parser::ParserConfig;
    fn show(&mut self, settings: &Settings, ui: &mut egui::Ui, input: &mut Self::Input) {
        ui.label("Configuration");
        for (index, (mut path, mut literal)) in input.iter().enumerate() {
            ui.push_id(index, |ui| {
                ui.horizontal(|ui| {
                    Path::default().show(settings, ui, &mut path);
                    Literal::default().show(settings, ui, &mut literal);
                    input.set(path, literal);
                });
            });
        }
    }
}